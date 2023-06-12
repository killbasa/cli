use anyhow::{anyhow, Result};
use clap::Args;

use crate::{app, config, internal::files};

/// Set the dotfiles repository path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The path to the dotfiles repository
    path: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        let path = files::resolve_path(&Some(self.path.clone()))
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?
            .display()
            .to_string();

        config.dotfiles = Some(path.clone());
        config::save(config)?;

        println!("Dotfiles path set to: {}", path);

        Ok(())
    }
}
