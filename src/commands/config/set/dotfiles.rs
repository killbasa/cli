use anyhow::{anyhow, Result};
use clap::Args;

use crate::{app, config, internal::files};

/// Set the dotfiles repo path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The path to the dotfiles repo
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

        display!("Dotfiles path set to: {}", path);

        Ok(())
    }
}
