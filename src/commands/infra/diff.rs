use anyhow::Result;
use clap::Args;

use crate::{app, internal::git};

/// Check if there are uncommited infra changes
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.infra {
            None => println!("No infra path set"),
            Some(path) => {
                let changes = git::uncommitted_changes(path)?;

                match changes {
                    0 => println!("Your infra repository is up to date"),
                    _ => println!("Your infra repository has uncommitted changes"),
                }
            }
        }

        Ok(())
    }
}
