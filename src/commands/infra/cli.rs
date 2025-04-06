use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage infra
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
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Diff(cli) => cli.run(),
            Commands::Find(cli) => cli.run(),
            Commands::Open(cli) => cli.run(),
        }
    }
}
