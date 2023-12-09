use anyhow::{anyhow, Result};
use clap::Args;
use git2::{Repository, StatusEntry};

use crate::app;

/// Check if there are uncommited dotfile changes
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.dotfiles {
            None => println!("No dotfiles path set"),
            Some(ref path) => {
                let repo = Repository::open(path)
                    .map_err(|err| anyhow!("failed to open repo: {}", err))?;

                let statuses = repo
                    .statuses(None) //
                    .map_err(|err| anyhow!("failed to get repo status: {}", err))?;

                let changed: Vec<StatusEntry> = statuses //
                    .iter()
                    .filter(|e| !e.status().is_ignored())
                    .collect();

                match changed.len() {
                    0 => println!("Your dotfiles repository is up to date"),
                    _ => println!("Your dotfiles repository has uncommitted changes"),
                }
            }
        }

        Ok(())
    }
}
