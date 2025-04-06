use anyhow::Result;
use clap::Args;
use git2::Config;
use serde::Deserialize;

/// Open pages for a Crate
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the crate
    name: String,
    /// If the docs should be opened
    #[arg(short, long)]
    docs: bool,
    /// If the GitHub repository should be opened
    #[arg(short, long)]
    github: bool,
}

#[derive(Debug, Deserialize)]
struct Crate {
    #[serde(rename = "crate")]
    _crate: Repository,
}

#[derive(Debug, Deserialize)]
struct Repository {
    repository: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        if !self.github && !self.docs {
            open::that(format!("https://crates.io/crates/{}", self.name))?;
        }

        if self.docs {
            open::that(format!("https://docs.rs/{}", self.name))?;
        }

        if self.github {
            let config = Config::open_default()?;
            let url = format!("https://crates.io/api/v1/crates/{}", self.name);

            // Crates.io needs a User-agent to be set
            let user_agent = match config.get_entry("user.email") {
                Ok(val) => format!("kb-cli ({})", val.value().unwrap()),
                Err(_) => "kb-cli".to_owned(),
            };

            let resp = reqwest::blocking::ClientBuilder::new()
                .build()?
                .get(url)
                .header("User-agent", user_agent)
                .send()?;

            if !resp.status().is_success() {
                open::that(format!("https://crates.io/crates/{}", self.name))?;
            } else {
                open::that(resp.json::<Crate>()?._crate.repository)?;
            }
        }

        Ok(())
    }
}
