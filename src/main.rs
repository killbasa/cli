#[macro_use]
mod macros;
mod app;
mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    app::set_global_verbosity(cli.verbose.log_level_filter());
    app::set_global_config(config::load()?);

    cli.exec()
}
