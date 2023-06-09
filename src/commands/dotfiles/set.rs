use anyhow::Result;
use clap::Args;

/// Set the dotfiles env var
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        display!("{}", "set");

        Ok(())
    }
}
