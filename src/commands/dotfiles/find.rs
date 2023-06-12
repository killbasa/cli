use anyhow::Result;
use clap::Args;

use crate::app;

/// Check the dotfiles path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.dotfiles {
            None => println!("No dotfiles path set"),
            Some(ref url) => println!("Dotfiles path: {}", url),
        }

        Ok(())
    }
}
