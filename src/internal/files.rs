use anyhow::Result;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn canonicalize_path(path: &String) -> String {
    match dunce::canonicalize(path) {
        Ok(p) => p.display().to_string(),
        Err(_) => path.to_string(),
    }
}

pub fn resolve_path(path: &Option<String>) -> Result<PathBuf> {
    let result: PathBuf = match path {
        Some(ref _path) => {
            let resolved_path = canonicalize_path(&_path);
            PathBuf::from(resolved_path)
        }
        None => env::current_dir()?,
    };

    return Ok(result);
}

pub fn copy_recursive<U: AsRef<Path>, V: AsRef<Path>>(
    from: U,
    to: V,
) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();

        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {}
                }
            }
        }
    }

    Ok(())
}
