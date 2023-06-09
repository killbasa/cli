use anyhow::Result;
use clap::Args;
use std::{fmt::Debug, fs};

use super::templates::Templates;

/// Clone a template
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The URL of the template
    #[arg(value_enum)]
    template: Templates,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match self.template {
            Templates::NodeTS => {
                let _ = fs::create_dir("./node-ts");
            }
        }

        Ok(())
    }
}
