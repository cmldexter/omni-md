use axum::{
    http::HeaderMap,
    Json,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    GitLab,
    GitHub,
    Unknown,
}

/// A unified webhook event abstracted from generic Git provider payloads.
#[derive(Debug, Clone)]
pub struct UnifiedPushEvent {
    pub provider: Provider,
    pub repository_url: String,
    pub default_branch: String,
    pub commits: Vec<CommitDelta>,
}

#[derive(Debug, Clone)]
pub struct CommitDelta {
    pub id: String,
    pub message: String,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

/// Takes Axum headers and raw JSON payloads to determine provider and extract unified event vectors.
pub fn parse_webhook(headers: &HeaderMap, payload: &Value) -> anyhow::Result<UnifiedPushEvent> {
    // 1. Detect Provider (GitLab vs GitHub)
    let provider = if headers.contains_key("x-gitlab-event") {
        Provider::GitLab
    } else if headers.contains_key("x-github-event") {
        Provider::GitHub
    } else {
        Provider::Unknown
    };

    // 2. Extract Data Agnostically
    match provider {
        Provider::GitLab => parse_gitlab_push(payload),
        Provider::GitHub => parse_github_push(payload),
        Provider::Unknown => anyhow::bail!("Unsupported or unknown webhook provider"),
    }
}

fn parse_gitlab_push(payload: &Value) -> anyhow::Result<UnifiedPushEvent> {
    // Extremely simplified GitLab push event unmarshalling (just enough for routing drafts)
    let repo_url = payload["project"]["http_url"].as_str().unwrap_or_default().to_string();
    let branch = payload["ref"].as_str().unwrap_or("refs/heads/main").replace("refs/heads/", "");
    
    let mut commits = Vec::new();
    if let Some(arr) = payload["commits"].as_array() {
        for c in arr {
            commits.push(CommitDelta {
                id: c["id"].as_str().unwrap_or_default().to_string(),
                message: c["message"].as_str().unwrap_or_default().to_string(),
                added: extract_string_array(&c["added"]),
                modified: extract_string_array(&c["modified"]),
                removed: extract_string_array(&c["removed"]),
            });
        }
    }

    Ok(UnifiedPushEvent {
        provider: Provider::GitLab,
        repository_url: repo_url,
        default_branch: branch,
        commits,
    })
}

fn parse_github_push(payload: &Value) -> anyhow::Result<UnifiedPushEvent> {
    // Structural placeholder to show Kubernetes/Cloud orchestration platform friendliness
    let repo_url = payload["repository"]["clone_url"].as_str().unwrap_or_default().to_string();
    let branch = payload["ref"].as_str().unwrap_or("refs/heads/main").replace("refs/heads/", "");
    
    // Exact same mapping as above essentially, but mapped to Github's strict payload
    Ok(UnifiedPushEvent {
        provider: Provider::GitHub,
        repository_url: repo_url,
        default_branch: branch,
        commits: vec![], // Add commit delta parsing later
    })
}

fn extract_string_array(val: &Value) -> Vec<String> {
    val.as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}
