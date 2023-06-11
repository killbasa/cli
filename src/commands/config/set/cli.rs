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
    Dotfiles(super::dotfiles::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Dotfiles(cli) => cli.exec(),
        }
    }
}
