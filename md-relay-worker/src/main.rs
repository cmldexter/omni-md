mod config;
mod db;
mod git_engine;
mod webhook;

use axum::{
    extract::{Json, State},
    http::HeaderMap,
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Define a placeholder Job struct for our Queue
#[derive(Debug)]
pub struct SyncJob {
    pub source: String,
    pub is_draft_sync: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize Logging (Kubernetes standard stdout)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    info!("Booting Omni-MD worker service...");

    // 2. Database Connection (SQLite)
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://omni.db".to_string());
    info!("SQLite configured at: {}", db_url);
    
    // Performance Tuned SQLite Pool Setup
    let pool = db::init_db(&db_url).await.expect("Failed to initialize tuned SQLite Database");

    // 3. Initialize MPSC Channel for Webhook Job Queue
    // Using a queue size of 100 handles massive concurrent spikes without crashing.
    let (tx, mut rx) = mpsc::channel::<SyncJob>(100);

    // 4. The Sequential Background Sync Worker
    // This thread acts as the "Consumer" preventing race condition Git writes
    tokio::spawn(async move {
        info!("Background Git Sync thread initialized.");
        while let Some(job) = rx.recv().await {
            info!("Processing sync job from queue: {:?}", job);
            if job.is_draft_sync {
                info!("Executing Safe-Sync Draft Mechanism (Wiki -> Source Draft)");
            } else {
                info!("Executing Standard Sync (Source -> Wiki Publish)");
            }
        }
    });

    // 5. Build Axum Router (K8s friendly)
    let app = Router::new()
        .route("/api/health/liveness", get(liveness_probe))
        .route("/api/health/readiness", get(readiness_probe))
        .route("/api/logs", get(get_logs_handler))
        .route("/webhook", post(webhook_handler))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(pool); // Serve connection cleanly to thread-handlers

    // 6. Bind to Port (0.0.0.0 for Docker containers)
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening for webhooks on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Start Web Server
    axum::serve(listener, app).await?;

    Ok(())
}

// Kubernetes Liveness: Is the binary actually running?
async fn liveness_probe() -> &'static str {
    "OK"
}

// Kubernetes Readiness: Is the Database/Queue connected and ready for traffic?
async fn readiness_probe(State(pool): State<SqlitePool>) -> axum::http::StatusCode {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => axum::http::StatusCode::OK,
        Err(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
    }
}

// React UI Dashboard Event Readout
async fn get_logs_handler(State(pool): State<SqlitePool>) -> Result<Json<Vec<db::SyncEvent>>, axum::http::StatusCode> {
    match db::get_history(&pool).await {
        Ok(events) => Ok(Json(events)),
        Err(e) => {
            tracing::error!("Failed to fetch history logs: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn webhook_handler(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> &'static str {
    // Generate a unique Request ID for End-to-End Tracing
    let req_id = uuid::Uuid::new_v4().to_string();
    let _span = tracing::info_span!("webhook_request", req_id = %req_id).entered();

    match webhook::parse_webhook(&headers, &payload) {
        Ok(event) => {
            tracing::info!(
                provider = ?event.provider,
                repository = %event.repository_url,
                "Parsed Webhook successfully"
            );
            // In reality, this endpoint will construct a SyncJob and push it into the MPSC queue `tx`
            "Webhook Queued Successfully"
        }
        Err(e) => {
            tracing::error!("Webhook signature/payload rejection: {}", e);
            "Invalid Payload or Unsupported Provider"
        }
    }
}
