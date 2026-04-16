use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::str::FromStr;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct SyncEvent {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub repository: String,
    pub target_file: String,
    pub status: String,
}

pub async fn init_db(db_url: &str) -> Result<SqlitePool> {
    // Performance Tuning: WAL Mode greatly increases concurrency and reduces locks
    let options = SqliteConnectOptions::from_str(db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal) // Safe for WAL and drastically faster
        .pragma("busy_timeout", "5000") // 5s lock timeout
        .pragma("cache_size", "-20000") // 20MB cache
        .pragma("mmap_size", "2147483648") // 2GB memory mapped capacity
        .pragma("foreign_keys", "ON");

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;

    // Create standard Schema footprint via ad-hoc queries (bypass heavy migration engines)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sync_history (
            id TEXT PRIMARY KEY,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            repository TEXT NOT NULL,
            target_file TEXT NOT NULL,
            status TEXT NOT NULL
        );"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn get_history(pool: &SqlitePool) -> Result<Vec<SyncEvent>> {
    let records = sqlx::query_as::<_, SyncEvent>(
        "SELECT id, timestamp, repository, target_file, status FROM sync_history ORDER BY timestamp DESC LIMIT 50"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(records)
}

pub async fn insert_event(pool: &SqlitePool, id: &str, repo: &str, file: &str, status: &str) -> Result<()> {
    sqlx::query(
        "INSERT INTO sync_history (id, repository, target_file, status) VALUES (?, ?, ?, ?)"
    )
    .bind(id)
    .bind(repo)
    .bind(file)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}
