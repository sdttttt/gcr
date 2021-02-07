use std::fmt::Error;

use crate::repo::Repository;

use super::CommitPlugin;

#[derive(Clone, Copy)]
pub struct PushPlugin;

impl PushPlugin {
	pub fn new() -> impl CommitPlugin {
		Self {}
	}
}

impl CommitPlugin for PushPlugin {
	fn before(&self, _: &Repository, _: &String) -> Option<Error> {
		println!("log plugin runing.");
		None
	}

	fn after(&self, _: &Repository, _: &String) -> Option<Error> {
		println!("log plugin runing.");
		None
	}
}
