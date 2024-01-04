use anyhow::{Ok, Result};
use clap::Args;

use crate::{app, internal::programs::open_in_vscode};

/// Open the infra repo in VSCode
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        match config.infra {
            None => println!("No infra path set"),
            Some(path) => {
                open_in_vscode(path)?;
            }
        }

        Ok(())
    }
}
