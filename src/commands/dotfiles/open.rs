use anyhow::{Ok, Result};
use clap::Args;

use crate::{config, internal::programs::open_in_vscode};

/// Open the dotfiles repo in VSCode
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let config = config::config();

        match &config.dotfiles {
            None => println!("No dotfiles path set"),
            Some(path) => {
                open_in_vscode(path.to_string())?;
            }
        }

        Ok(())
    }
}
