/* utility.rs */

use std::path::PathBuf;
use std::process::exit;
use std::fs;

pub fn try_read_file(file: &Option<PathBuf>) -> String {
    let path_string = match file {
        Some(p) => format!("{}", p.to_string_lossy().into_owned()),
        None => String::from(""),
    };

    match fs::read_to_string(path_string.clone()) {
        Err(why) => {
            eprintln!("cannot read '{}': {}", path_string, why);
            exit(1);
        },
        Ok(content) => content,
    }
}
