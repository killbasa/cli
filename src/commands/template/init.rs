use anyhow::{anyhow, Result};
use clap::Args;
use git2::Repository;
use spinoff::{spinners, Color, Spinner};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use super::templates::Templates;
use crate::internal::files;

/// Clone a template
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The template to initiate
    #[arg(value_enum)]
    template: Templates,
    /// The output location
    path: Option<String>,
    /// If a git repository should be initialized
    #[arg(short, long)]
    git: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let spinner = Spinner::new(spinners::Dots, "Cloning template...", Color::Blue);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| anyhow!("error when getting time"))?
            .as_millis();

        let path = files::resolve_path(&self.path)
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?;

        if path.is_dir() {
            spinner.fail("Failed to clone template");
            return Err(anyhow!("there is already a directory at that location"));
        }

        let temp_path = format!("{}/{}", path.display(), now);

        Repository::clone("https://github.com/killbasa/templates", &temp_path)
            .map_err(|e| anyhow!("failed to clone repo: {}", e))?;

        files::copy_recursive(
            format!("{}/templates/{}", temp_path, self.template.to_string()),
            &path,
        )
        .map_err(|e| anyhow!("failed to copy files: {}", e))?;

        fs::remove_dir_all(temp_path)?;

        if self.git {
            Repository::init(path) //
                .map_err(|e| anyhow!("failed to initialize git repo: {}", e))?;
        }

        spinner.success(&format!("Template \"{}\" cloned", self.template));

        Ok(())
    }
}
