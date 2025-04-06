use anyhow::Result;
use clap::Args;

use crate::{config, internal::git};

/// Check if there are uncommited dotfile changes
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let config = config::config();

        match &config.dotfiles {
            None => println!("No dotfiles path set"),
            Some(path) => {
                let changes = git::uncommitted_changes(path.to_string())?;

                match changes {
                    0 => println!("Your dotfiles repository is up to date"),
                    _ => println!("Your dotfiles repository has uncommitted changes"),
                }
            }
        }

        Ok(())
    }
}
