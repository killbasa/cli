use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

use crate::Cli as RootCli;

/// Generate autocompletions.
/// Example: `kb complete fish > ~/.config/fish/completions/kb.fish`
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The shell to generate autocompletions for
    #[arg(value_enum)]
    shell: Shell,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut cmd = RootCli::command();
        let bin_name = cmd.get_name().to_string();

        generate(self.shell, &mut cmd, bin_name, &mut io::stdout());

        Ok(())
    }
}
