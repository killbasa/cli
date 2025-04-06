use anyhow::Result;
use clap::Args;
use git2::Config;
use serde::Deserialize;

/// Open pages for an NPM package
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the package
    name: String,
    /// If the GitHub repository should be opened
    #[arg(short, long)]
    github: bool,
}

#[derive(Debug, Deserialize)]
struct Package {
    repository: Repository,
}

#[derive(Debug, Deserialize)]
struct Repository {
    #[serde(rename = "type")]
    _type: String,
    url: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        if self.github {
            let config = Config::open_default()?;
            let url = format!("https://registry.npmjs.org/{}", self.name);

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
                open::that(format!("https://www.npmjs.com/package/{}", self.name))?;
                return Ok(());
            }

            let json = resp.json::<Package>()?;

            let repo_url = match json.repository._type.as_str() {
                "git" => json.repository.url.replace("git+", ""),
                _ => json.repository.url,
            };

            open::that(repo_url)?;
        } else {
            open::that(format!("https://www.npmjs.com/package/{}", self.name))?;
        }

        Ok(())
    }
}
