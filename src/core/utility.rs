/* utility.rs */

use std::path::PathBuf;
use std::fs;

pub fn try_read_file(file: &Option<PathBuf>) -> Result<String, String> {
    let path_string = match file {
        Some(p) => format!("{}", p.to_string_lossy().into_owned()),
        None => String::from(""),
    };

    match fs::read_to_string(path_string.clone()) {
        Ok(content) => Ok(content),
        Err(why) => Err(format!("cannot read '{}': {}", path_string, why)),
    }
}
