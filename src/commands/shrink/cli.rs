use anyhow::{anyhow, Result};
use byte_unit::{Byte, UnitType};
use clap::Args;
use glob::{MatchOptions, Pattern};
use spinoff::{spinners, Color, Spinner};
use std::{fs, path::PathBuf};

use crate::internal::files;

use super::patterns::FILE_NAMES;

/// Shrink a black hole
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The path to the black hole
    path: Option<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut spinner = Spinner::new(
            spinners::Dots,
            "Starting astral manipulation...",
            Color::Blue,
        );

        let path = files::resolve_path(&self.path, Some("node_modules"))
            .map_err(|e| anyhow!("failed to resolve path: {}", e))?;

        if !path.is_dir() || !files::is_file_name_eq(&path, "node_modules") {
            spinner.fail("Shrink failed");
            return Err(anyhow!("invalid node_modules directory"));
        }

        let before =
            Byte::from_u64(files::dir_size(&path)?).get_appropriate_unit(UnitType::Decimal);

        let (total, deleted) = delete_files_in_pattern(&path)?;

        let after = Byte::from_u64(files::dir_size(&path)?).get_appropriate_unit(UnitType::Decimal);

        spinner.success(&format!(
            "Black hole has been shrunk ({:.2} -> {:.2})\n{} files deleted out of {} total files",
            before, after, deleted, total
        ));

        Ok(())
    }
}

fn delete_files_in_pattern(path: &PathBuf) -> Result<(i32, i32)> {
    let mut dir_stack = vec![path.to_owned()];
    let mut total = 0;
    let mut deleted = 0;

    while let Some(path) = dir_stack.pop() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                total += 1;

                let result = FILE_NAMES.into_iter().any(|pattern| {
                    let pattern = Pattern::new(pattern).unwrap();
                    pattern.matches_with(
                        entry.file_name().to_str().unwrap(),
                        MatchOptions {
                            case_sensitive: false,
                            require_literal_separator: false,
                            require_literal_leading_dot: false,
                        },
                    )
                });

                if result {
                    if entry.metadata()?.is_file() {
                        fs::remove_file(entry_path)?;
                        deleted += 1;
                    } else {
                        fs::remove_dir_all(entry_path)?;
                        deleted += 1;
                    }
                } else if entry.metadata()?.is_dir() {
                    dir_stack.push(entry_path);
                }
            }
        }
    }

    Ok((total, deleted))
}
