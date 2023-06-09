use anyhow::Result;
use clap::Args;

/// Sync your local dotfiles
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        display!("{}", "sync");

        Ok(())
    }
}
