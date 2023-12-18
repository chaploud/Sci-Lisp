use std::path::PathBuf;
use std::process::exit;
use std::fs::File;

pub fn is_file_exist(file: &Option<PathBuf>) {
    let path_string = match file {
        Some(p) => format!("{}", p.to_string_lossy().into_owned()),
        None => String::from(""),
    };

    match File::open(&path_string) {
        Err(why) => {
            println!("couldn't open '{}': {}", path_string, why);
            exit(1);
        },
        Ok(_) => (),
    };
}
