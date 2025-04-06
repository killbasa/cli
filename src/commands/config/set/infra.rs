use anyhow::{Result, anyhow};
use clap::Args;

use crate::{config, internal::files};

/// Set the infra repository path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The path to the infra repository
    path: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut config = config::config().clone();

        let path = files::resolve_path(&Some(self.path.clone()), None)
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?
            .display()
            .to_string();

        config.infra = Some(path.clone());
        config::save(config)?;

        println!("Infra path set to: {}", path);

        Ok(())
    }
}
