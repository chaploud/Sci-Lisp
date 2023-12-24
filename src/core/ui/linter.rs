/* core/ui/linter.rs */

use std::path::PathBuf;

use crate::core::types::error::Result;
use crate::core::utility::utility::try_read_file;

pub fn lint(file: Option<PathBuf>) -> Result<()> {
    print!("Linting '{}' ...", file.clone().unwrap().to_string_lossy());
    let _content = try_read_file(&file)?;
    print!("Done.");
    Ok(())
}
