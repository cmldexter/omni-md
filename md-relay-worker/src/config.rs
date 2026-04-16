use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmniConfig {
    pub config: ConfigSettings,
    pub sync: Vec<SyncMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSettings {
    pub listen_port: u16,
    pub listen_host: String,
    pub target_repo: String,
    pub target_branch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncMapping {
    pub source_repo: String,
    pub source_branch: String,
    pub mappings: Vec<FileMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMapping {
    pub src: String,
    pub dest: String,
}

impl OmniConfig {
    /// Loads the configuration from standard YAML string
    pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
        let config: OmniConfig = serde_yaml::from_str(content)?;
        Ok(config)
    }
}
