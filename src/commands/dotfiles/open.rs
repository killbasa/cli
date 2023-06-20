use std::process::Command;

use anyhow::{anyhow, Ok, Result};
use clap::Args;

use crate::{app, internal::programs::is_program_in_path};

/// Check the dotfiles path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.dotfiles {
            None => println!("No dotfiles path set"),
            Some(ref path) => {
                if !is_program_in_path("code") {
                    return Err(anyhow!("VSCode is not installed"));
                }

                Command::new("code")
                    .arg(path)
                    .output()
                    .map_err(|e| anyhow!("Failed to open dotfiles repo: {}", e))?;
            }
        }

        Ok(())
    }
}
