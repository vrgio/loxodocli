use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub access_token: String,
}

impl Config {
    fn config_path() -> Result<PathBuf> {
        let path = dirs::config_dir()
            .context("invalid config path")?
            .join("loxodocli")
            .join("settings.toml");
        Ok(path)
    }

    pub fn load() -> Result<Config> {
        let path = Self::config_path()?;
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Run setup with loxodocli init"))?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("Failed to make config directory {}", parent.display()))?;
        }
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(&path, toml_str).with_context(|| format!("Failed to write token to {}", path.display()))?;
        Ok(())
    }
}

