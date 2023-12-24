/* core/ui/compiler.rs */

use std::path::PathBuf;

use crate::core::utility::utility::try_read_file;

pub fn compile(file: Option<PathBuf>) -> Result<(), String> {
    print!(
        "Compiling '{}' ... ",
        file.clone().unwrap().to_string_lossy()
    );
    let _content = try_read_file(&file)?;
    println!("Done.");
    Ok(())
}
