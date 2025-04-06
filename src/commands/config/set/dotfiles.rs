use anyhow::{Result, anyhow};
use clap::Args;

use crate::{config, internal::files};

/// Set the dotfiles repository path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The path to the dotfiles repository
    path: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut config = config::config().clone();

        let path = files::resolve_path(&Some(self.path.clone()), None)
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?
            .display()
            .to_string();

        config.dotfiles = Some(path.clone());
        config::save(config)?;

        println!("Dotfiles path set to: {}", path);

        Ok(())
    }
}
