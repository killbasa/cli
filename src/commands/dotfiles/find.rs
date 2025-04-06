use anyhow::Result;
use clap::Args;

use crate::config;

/// Check the dotfiles path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let config = config::config();

        match config.dotfiles {
            None => println!("No dotfiles path set"),
            Some(ref path) => println!("{}", path),
        }

        Ok(())
    }
}
