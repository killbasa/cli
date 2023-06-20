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
    Diff(super::diff::Cli),
    Find(super::find::Cli),
    Open(super::open::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Diff(cli) => cli.exec(),
            Commands::Find(cli) => cli.exec(),
            Commands::Open(cli) => cli.exec(),
        }
    }
}
