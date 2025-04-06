mod commands;
mod config;
mod internal;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "A personal CLI to make life a little bit easier")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Clone(commands::clone::Cli),
    Complete(commands::complete::Cli),
    Config(commands::config::cli::Cli),
    Dotfiles(commands::dotfiles::cli::Cli),
    Infra(commands::infra::cli::Cli),
    Open(commands::open::cli::Cli),
    Template(commands::template::cli::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    config::set_global_config(config::load()?);

    match &cli.command {
        Commands::Clone(cli) => cli.run(),
        Commands::Complete(cli) => cli.run(),
        Commands::Config(cli) => cli.run(),
        Commands::Dotfiles(cli) => cli.run(),
        Commands::Infra(cli) => cli.run(),
        Commands::Open(cli) => cli.run(),
        Commands::Template(cli) => cli.run(),
    }
}
