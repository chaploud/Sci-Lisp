use std::path::{Path, PathBuf};

pub fn is_file_exist(file: &Option<PathBuf>) -> bool {
    let path_string = match file {
        Some(p) => format!("{}", p.to_string_lossy().into_owned()),
        None => String::from(""),
    };

    let is_existed = Path::new(&path_string).exists();
    if !is_existed {
        println!("file '{}' does not exist!", path_string);
        return false;
    } else {
        return true;
    }
}
