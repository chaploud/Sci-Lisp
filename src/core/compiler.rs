use crate::core::common::is_file_exist;
use std::path::PathBuf;

pub fn compile(file: Option<PathBuf>) {
    println!("Compiling {}...", file.clone().unwrap().to_string_lossy());
    if !is_file_exist(&file) {
        return;
    }
}
