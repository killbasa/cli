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
    /// Force overwrite the target path
    #[arg(short, long)]
    force: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut spinner = Spinner::new(
            spinners::Dots, //
            "Cloning template...",
            Color::Blue,
        );

        let target_path = files::resolve_path(&self.path, None)
            .map_err(|err| anyhow!("failed to resolve path: {}", err))?;

        if self.force && target_path.exists() {
            fs::remove_dir_all(&target_path)
                .map_err(|err| anyhow!("failed to remove target path: {}", err))?;
        } else if target_path.is_dir() && !target_path.read_dir()?.next().is_none() {
            spinner.fail("Failed to clone template");
            return Err(anyhow!("there are already files at that location"));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| anyhow!("error when getting time: {}", err))?
            .as_millis();
        let temp_path = format!("{}/.{}", target_path.display(), now);

        Repository::clone("https://github.com/killbasa/templates", &temp_path)
            .map_err(|err| anyhow!("failed to clone repo: {}", err))?;

        files::copy_recursive(
            format!("{}/templates/{}", temp_path, self.template.to_string()),
            &target_path,
        )
        .map_err(|err| anyhow!("failed to copy files: {}", err))?;

        fs::remove_dir_all(temp_path)
            .map_err(|err| anyhow!("failed to remove temporary directory: {}", err))?;

        spinner.success(&format!("Template \"{}\" cloned", self.template));

        Ok(())
    }
}
