/* repl.rs */

use crate::core::utility::try_read_file;
use std::path::PathBuf;

use crate::core::parser::parse;

pub fn repl() {
    println!("Sci-Lisp REPL\n");
}

pub fn execute(file: Option<PathBuf>) {
    println!("Executing '{}' ...", file.clone().unwrap().to_string_lossy());
    let content = try_read_file(&file);
    println!("{:#?}", parse(&content));

}
