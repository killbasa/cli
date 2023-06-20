use std::{env, fs};

pub fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for value in path.split(":") {
            let program_path = format!("{}/{}", value, program);
            if fs::metadata(program_path).is_ok() {
                return true;
            }
        }
    }

    false
}
