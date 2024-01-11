/* core/utility.rs */

use std::fs;
use std::path::PathBuf;

use crate::core::types::error::Error;
use crate::core::types::error::Result;

pub fn try_read_file(file: &Option<PathBuf>) -> Result<String> {
    let path_string = match file {
        Some(p) => p.to_string_lossy().into_owned().to_string(),
        None => String::from(""),
    };

    match fs::read_to_string(path_string.clone()) {
        Ok(content) => Ok(content),
        Err(why) => {
            eprintln!("cannot read '{}'", path_string);
            Err(Error::IO(why))
        }
    }
}
