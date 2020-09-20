use std::{fs, io};

pub fn current_path() -> String {
    let path = fs::canonicalize(".").unwrap();
    String::from(path.to_str().unwrap())
}