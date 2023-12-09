use anyhow::{anyhow, Result};
use clap::Args;
use git2::Repository;
use spinoff::{spinners, Color, Spinner};
use std::fs;
use url::Url;

use crate::internal::files;

/// Clone a repository
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The URL of the repository
    url: String,
    /// The output location
    path: Option<String>,
    /// Force overwrite the target path
    #[arg(short, long)]
    force: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut spinner = Spinner::new(spinners::Dots, "Cloning repository...", Color::Blue);

        let url = Url::parse(&self.url)
            .map_err(|_| anyhow!("please provide a valid URL"))?
            .to_string();

        if !url.ends_with(".git") {
            spinner.fail("Failed to clone repository");
            return Err(anyhow!("make sure the URL ends with \".git\""));
        }

        let target_path = files::resolve_path(&self.path, None)
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?;

        if self.force && target_path.exists() {
            fs::remove_dir_all(&target_path)
                .map_err(|err| anyhow!("failed to remove target path: {}", err))?;
        } else if target_path.is_dir() && !target_path.read_dir()?.next().is_none() {
            spinner.fail("Failed to clone template");
            return Err(anyhow!("there are already files at that location"));
        }

        Repository::clone(&url, &target_path) //
            .map_err(|e| anyhow!("failed to clone repo: {}", e))?;

        spinner.success("Repository cloned");

        Ok(())
    }
}
