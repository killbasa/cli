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
    Infra(super::infra::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Dotfiles(cli) => cli.run(),
            Commands::Infra(cli) => cli.run(),
        }
    }
}
