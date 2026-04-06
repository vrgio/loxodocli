use std::fs;
use std::path::PathBuf;
use dirs::config_dir;
use anyhow::{Context, Result};
use toml::Value;

pub struct Config {
    pub url: String,
    pub token: String,
}

impl Config {
    pub fn load() -> std::io::Result<()> {
        let config_file = config_dir()
            .map(|p| p.join("loxodocli").join("settings.toml"))
            .expect("not found");

        let contents = fs::read_to_string(&config_file)?;
        let value: Value = contents.parse().expect("Invalid TOML");

        println!("Full config: {:#?}", value);

        Ok(())
    }

}
