use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub default_preset: Option<String>,
}

fn get_config_path() -> Result<PathBuf> {
    let config_path_str = "~/.config/muffin/";
    let expanded_path = shellexpand::full(config_path_str)?.into_owned();
    let config_dir = PathBuf::from(expanded_path);
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(Config::default());
    }
    let config_str = fs::read_to_string(config_path)?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let config_str = toml::to_string(config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}
