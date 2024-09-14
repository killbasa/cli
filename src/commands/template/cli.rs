use anyhow::Result;
use clap::{Args, Subcommand};

/// Generate a repository from a template
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init(super::init::Cli),
    List(super::list::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Init(cli) => cli.exec(),
            Commands::List(cli) => cli.exec(),
        }
    }
}
