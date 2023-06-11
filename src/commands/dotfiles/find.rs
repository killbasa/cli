use anyhow::Result;
use clap::Args;

use crate::app;

/// Check if the dotfiles path is set
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.dotfiles {
            None => display!("No dotfiles path is set"),
            Some(ref url) => display!("Dotfiles path: {}", url),
        }

        Ok(())
    }
}
