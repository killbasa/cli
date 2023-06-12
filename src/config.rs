use anyhow::{Context, Result};
use confy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const APP_NAME: &str = "kb";
const FILE_STEM: &str = "config";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub email: Option<String>,
    pub dotfiles: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            email: None,
            dotfiles: None,
        }
    }
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
