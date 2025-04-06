use anyhow::{Result, anyhow};
use clap::Args;
use std::process::{Command, Stdio};

use crate::config;

#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let path = config::path()?;

        let output = Command::new("cat")
            .arg(path)
            .stdout(Stdio::piped())
            .output()
            .map(|o| String::from_utf8(o.stdout).unwrap())
            .map_err(|e| anyhow!("Failed to display config: {}", e))?;

        println!("{}", output);

        Ok(())
    }
}
