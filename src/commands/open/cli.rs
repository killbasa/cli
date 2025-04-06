use anyhow::Result;
use clap::{Args, Subcommand};

/// Open a package's home page
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Node(super::node::Cli),
    Rust(super::rust::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Node(cli) => cli.run(),
            Commands::Rust(cli) => cli.run(),
        }
    }
}
