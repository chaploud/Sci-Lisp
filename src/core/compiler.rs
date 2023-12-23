/* compiler.rs */

use crate::core::utility::try_read_file;
use std::path::PathBuf;

pub fn compile(file: Option<PathBuf>) -> Result<(), String> {
    print!(
        "Compiling '{}' ... ",
        file.clone().unwrap().to_string_lossy()
    );
    let _content = try_read_file(&file)?;
    println!("Done.");
    Ok(())
}
