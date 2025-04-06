use anyhow::{Result, anyhow};
use git2::{Repository, StatusEntry};

pub fn uncommitted_changes(path: String) -> Result<usize> {
    let repo = Repository::open(path) //
        .map_err(|err| anyhow!("failed to open repo: {}", err))?;

    let statuses = repo
        .statuses(None) //
        .map_err(|err| anyhow!("failed to get repo status: {}", err))?;

    let changed: Vec<StatusEntry> = statuses //
        .iter()
        .filter(|e| !e.status().is_ignored())
        .collect();

    Ok(changed.len())
}
