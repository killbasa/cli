use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage dotfiles
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check(super::check::Cli),
    Set(super::set::Cli),
    Sync(super::sync::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Check(cli) => cli.exec(),
            Commands::Set(cli) => cli.exec(),
            Commands::Sync(cli) => cli.exec(),
        }
    }
}
