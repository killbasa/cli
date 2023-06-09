use anyhow::Result;
use clap::Args;
use std::env;

use super::DOTFILES_REPO_KEY;

/// Check if the dotfiles env var is set
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match env::var(DOTFILES_REPO_KEY) {
            Ok(v) => display!("{}: {}", DOTFILES_REPO_KEY, v),
            Err(e) => panic!("${} is not set ({})", DOTFILES_REPO_KEY, e),
        }

        Ok(())
    }
}
