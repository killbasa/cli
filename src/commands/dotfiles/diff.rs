use anyhow::Result;
use clap::Args;
use git2::{Repository, StatusOptions};

use crate::app;

/// Check if the dotfiles path is set
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.dotfiles {
            None => display!("No dotfiles path set"),
            Some(ref dotfiles) => {
                let repo = Repository::open(dotfiles)?;

                let mut opts = StatusOptions::new();
                let statuses = repo.statuses(Some(&mut opts))?;

                match statuses.len() {
                    0 => display!("Your dotfiles repository is up to date"),
                    _ => display!("Your dotfiles repository has uncommitted changes"),
                }
            }
        }

        Ok(())
    }
}
