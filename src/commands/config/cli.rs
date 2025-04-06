use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage the config file
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Find(super::find::Cli),
    Set(super::set::cli::Cli),
    Print(super::print::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Find(cli) => cli.run(),
            Commands::Set(cli) => cli.run(),
            Commands::Print(cli) => cli.run(),
        }
    }
}
