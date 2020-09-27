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
            Status::INDEX_NEW |
            Status::INDEX_MODIFIED |
            Status::INDEX_DELETED |
            Status::INDEX_RENAMED |
            Status::INDEX_TYPECHANGE
            => {
                tip = true;
                break;
            },
            _ => {}
        }
    }
    tip
}


pub fn remove_pound_prefix(input: &str) -> &str {
    match input.find("#") {
        Some(index) => {
            match input.get(index+1..input.len()) {
                Some(s) => s,
                _ => input,
            }
        } 
        _ => input,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string_start_with() {
        let str_1 = "#123";
        let result = "123";

        let str_2 = remove_pound_prefix(str_1);
        assert_eq!(str_2, result);
    }
}
