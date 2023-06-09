use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::commands::*;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "A personal CLI to make life a little bit easier")]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Complete(complete::Cli),
    Config(config::Cli),
    Dotfiles(dotfiles::cli::Cli),
    Generate(generate::cli::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Complete(cli) => cli.exec(),
            Commands::Config(cli) => cli.exec(),
            Commands::Dotfiles(cli) => cli.exec(),
            Commands::Generate(cli) => cli.exec(),
        }
    }
}
