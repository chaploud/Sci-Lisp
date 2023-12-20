/* linter.rs */

use crate::core::utility::try_read_file;
use std::path::PathBuf;

pub fn lint(file: Option<PathBuf>) {
    println!("Linting '{}' ...", file.clone().unwrap().to_string_lossy());
    let _content = try_read_file(&file);
}
