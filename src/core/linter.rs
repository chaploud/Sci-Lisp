/* linter.rs */

use crate::core::utility::is_file_exist;
use std::path::PathBuf;

pub fn lint(file: Option<PathBuf>) {
    println!("Linting '{}' ...", file.clone().unwrap().to_string_lossy());
    is_file_exist(&file);
}
