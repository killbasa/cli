use anyhow::Result;
use clap::Args;
use git2::Repository;

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
                let repo = Repository::open(path)?;

                let statuses = repo.statuses(None)?;

                match statuses.len() {
                    0 => println!("Your dotfiles repository is up to date"),
                    _ => println!("Your dotfiles repository has uncommitted changes"),
                }
            }
        }

        Ok(())
    }
}
