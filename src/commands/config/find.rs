use anyhow::Result;
use clap::Args;

use crate::config;

/// Check the config path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        println!("Config path: {}", config::path()?.display());

        Ok(())
    }
}
