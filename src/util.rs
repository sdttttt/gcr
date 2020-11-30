use git2::{Error, Signature, Status, Statuses};
use std::{env, fs};

use crate::metadata::{GIT_AUTHOR_EMAIL, GIT_AUTHOR_NAME, GIT_COMMITTER_EMAIL, GIT_COMMITTER_NAME};

pub fn current_path() -> String {
	let path = fs::canonicalize(".").unwrap();
	String::from(path.to_str().unwrap())
}

pub fn is_all_workspace(statuses: &Statuses) -> bool {
	let mut tip = false;
	for state in statuses.iter() {
		match state.status() {
			| Status::INDEX_NEW
			| Status::INDEX_MODIFIED
			| Status::INDEX_DELETED
			| Status::INDEX_RENAMED
			| Status::INDEX_TYPECHANGE => {
				tip = true;
				break;
			}
			| _ => {}
		}
	}
	tip
}

pub fn remove_pound_prefix(input: &str) -> &str {
	match input.find("#") {
		| Some(index) => match input.get(index + 1..input.len()) {
			| Some(s) => s,
			| _ => input,
		},
		| _ => input,
	}
}

pub fn vec_str_to_string(vec: Vec<&str>) -> Vec<String> {
	let mut result = vec![];
	for s in vec {
		result.push(String::from(s));
	}
	result
}

pub fn author_sign_from_env() -> Result<Signature<'static>, ()> {
	let name = match env::var(GIT_AUTHOR_NAME) {
		| Ok(v) => {
			if v.len() < 1 {
				return Err(());
			}
			v
		}
		| Err(_) => return Err(()),
	};

	let email = match env::var(GIT_AUTHOR_EMAIL) {
		| Ok(v) => {
			if v.len() < 1 {
				return Err(());
			}
			v
		}
		| Err(_) => return Err(()),
	};

	let sign = Signature::now(name.as_str(), email.as_str()).expect(
		"An error occurred while using the `GIT_AUTHOR_[USER, EMAIL]` to generate the commit sign.",
	);

	Ok(sign)
}

pub fn committer_sign_from_env() -> Result<Signature<'static>, ()> {
	let name = match env::var(GIT_COMMITTER_NAME) {
		| Ok(v) => {
			if v.len() < 1 {
				return Err(());
			}
			v
		}
		| Err(_) => return Err(()),
	};

	let email = match env::var(GIT_COMMITTER_EMAIL) {
		| Ok(v) => {
			if v.len() < 1 {
				return Err(());
			}
			v
		}
		| Err(_) => return Err(()),
	};

	let sign = Signature::now(name.as_str(), email.as_str()).expect(
		"An error occurred while using the environment variable to generate the commit sign.",
	);

	Ok(sign)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn get_current_path() {
		let path = current_path();
		assert!(path.len() > 0);
	}

	#[test]
	fn string_start_with() {
		let str_1 = "#123";
		let result = "123";

		let str_2 = remove_pound_prefix(str_1);
		assert_eq!(str_2, result);

		let str_3 = "321";
		let result_2 = "321";

		let str_4 = remove_pound_prefix(str_3);
		assert_eq!(str_4, result_2);
	}

	#[test]
	fn test_vec_str_to_string() {
		let one = "1";
		let two = "2";

		let v1 = vec![one, two];
		let mut v2 = vec_str_to_string(v1);
		assert_eq!(v2.pop().unwrap().as_str(), two);
		assert_eq!(v2.pop().unwrap().as_str(), one);
	}

	#[test]
	fn it_committer_sign_from_null_env() {
		env::set_var(GIT_COMMITTER_NAME, "");
		env::set_var(GIT_COMMITTER_EMAIL, "");

		if let Err(e) = committer_sign_from_env() {
			assert_eq!(e, ());
		} else {
			panic!("WHAT?")
		}
	}

	#[test]
	fn it_committer_sign_from_env() {
		let name = "sdttttt";
		let email = "sdttttt@sdttttt.sdttttt";
		env::set_var(GIT_COMMITTER_NAME, name);
		env::set_var(GIT_COMMITTER_EMAIL, email);

		if let Ok(sign) = committer_sign_from_env() {
			assert_eq!(sign.name().unwrap(), name);
			assert_eq!(sign.email().unwrap(), email);
		} else {
			panic!("WHAT?")
		}
	}

	#[test]
	fn it_author_sign_from_null_env() {
		env::set_var(GIT_AUTHOR_NAME, "");
		env::set_var(GIT_AUTHOR_EMAIL, "");

		if let Err(e) = author_sign_from_env() {
			assert_eq!(e, ());
		} else {
			panic!("WHAT?")
		}
	}

	#[test]
	fn it_author_sign_from_env() {
		let name = "sdttttt";
		let email = "sdttttt@sdttttt.sdttttt";
		env::set_var(GIT_AUTHOR_NAME, name);
		env::set_var(GIT_AUTHOR_EMAIL, email);

		if let Ok(sign) = author_sign_from_env() {
			assert_eq!(sign.name().unwrap(), name);
			assert_eq!(sign.email().unwrap(), email);
		} else {
			panic!("WHAT?")
		}
	}
}
