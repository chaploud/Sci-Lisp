use crate::core::common::is_file_exist;
use std::path::PathBuf;

pub fn repl() {
    println!("REPL");
}

pub fn execute(file: Option<PathBuf>) {
    println!("Executing {}...", file.clone().unwrap().to_string_lossy());
    if !is_file_exist(&file) {
        return;
    }
}
