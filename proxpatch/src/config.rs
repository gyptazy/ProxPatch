use std::fs;
use crate::models::{Config};
use log::{info, debug, warn, error};

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    debug!("→ Attempting to load config from: {}", path);

    let content = fs::read_to_string(path).map_err(|e| {
        error!("✗ Failed to read config file '{}': {}", path, e);
        e
    })?;
    debug!("✓ Config file '{}' successfully read ({} bytes)", path, content.len());

    let config: Config = serde_yaml::from_str(&content).map_err(|e| {
        error!("✗ Failed to parse YAML config '{}': {}", path, e);
        e
    })?;
    info!("✓ Successfully loaded and parsed config from: {}", path);

    Ok(config)
}