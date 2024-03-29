mod app;
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
    Shrink(commands::shrink::cli::Cli),
    Template(commands::template::cli::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    app::set_global_config(config::load()?);

    match &cli.command {
        Commands::Clone(cli) => cli.exec(),
        Commands::Complete(cli) => cli.exec(),
        Commands::Config(cli) => cli.exec(),
        Commands::Dotfiles(cli) => cli.exec(),
        Commands::Infra(cli) => cli.exec(),
        Commands::Open(cli) => cli.exec(),
        Commands::Shrink(cli) => cli.exec(),
        Commands::Template(cli) => cli.exec(),
    }
}
