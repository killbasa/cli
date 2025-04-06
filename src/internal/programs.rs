use anyhow::{Ok, Result, anyhow};
use std::{env, process::Command};

pub fn is_program_in_path(program: &str) -> bool {
    match env::var_os("PATH") {
        None => false,
        Some(paths) => env::split_paths(&paths).map(|p| p.join(program)).any(|p| p.exists()),
    }
}

pub fn open_in_vscode(path: String) -> Result<()> {
    if is_program_in_path("code") {
        let output = Command::new("code")
            .arg(path)
            .output()
            .map_err(|e| anyhow!("failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!("failed to execute command: {}", String::from_utf8_lossy(&output.stderr)))
        }
    } else {
        Err(anyhow!("VSCode is not installed"))
    }
}
