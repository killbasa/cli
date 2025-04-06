use anyhow::Result;
use clap::Args;

use crate::commands::template::templates::Templates;

/// List available templates
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        println!("Available templates:");
        println!("{}", Templates::Bare);
        println!("{}", Templates::NodeTS);
        println!("{}", Templates::Svelte);

        Ok(())
    }
}
