use anyhow::{anyhow, Result};
use clap::Args;
use git2::Repository;
use spinoff::{spinners, Color, Spinner};
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
    /// Intialize a git repository
    #[arg(short, long)]
    git: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let spinner = Spinner::new(spinners::Dots, "Cloning repository...", Color::Blue);

        let url = Url::parse(&self.url)
            .map_err(|_| anyhow!("please provide a valid URL"))?
            .to_string();

        let path = files::resolve_path(&self.path)
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?;

        Repository::clone(&url, &path) //
            .map_err(|e| anyhow!("failed to clone repo: {}", e))?;

        if self.git {
            Repository::init(path) //
                .map_err(|e| anyhow!("failed to initialize git repo: {}", e))?;
        }

        spinner.success("Repository cloned");

        Ok(())
    }
}
