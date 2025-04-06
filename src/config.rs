use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const APP_NAME: &str = "kb";
const FILE_STEM: &str = "kb.config";

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    pub email: Option<String>,
    pub dotfiles: Option<String>,
    pub infra: Option<String>,
}

pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, FILE_STEM)
        .with_context(|| "unable to find the config file")
}

pub fn load() -> Result<Config> {
    confy::load(APP_NAME, FILE_STEM).with_context(|| "unable to load config")
}

pub fn save(config: Config) -> Result<()> {
    confy::store(APP_NAME, FILE_STEM, config).with_context(|| "unable to save config")
}

pub fn config() -> &'static Config {
    CONFIG.get().expect("config is not initialized")
}

pub fn set_global_config(config: Config) {
    CONFIG.set(config).expect("could not set config")
}
