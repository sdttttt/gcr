use git2::{
    Statuses,
    Status
};
use std::fs;

pub fn current_path() -> String {
    let path = fs::canonicalize(".").unwrap();
    String::from(path.to_str().unwrap())
}

pub fn is_all_workspace(statuses: &Statuses) -> bool {
    let mut tip = false;
    for state in statuses.iter() {
        match state.status() {
            Status::INDEX_NEW => {
                tip = true;
                break;
            },
            Status::INDEX_MODIFIED => {
                tip = true;
                break;
            },
            Status::INDEX_DELETED => {
                tip = true;
                break;
            },
            Status::INDEX_RENAMED => {
                tip = true;
                break;
            },
            Status::INDEX_TYPECHANGE => {
                tip = true;
                break;
            },
            _ => {}
        }
    }
    tip
}