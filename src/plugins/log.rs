use git2::Error;

use crate::repo::Repository;

use super::CommitPlugin;

#[derive(Clone, Copy)]
pub struct LogPlugin;

impl LogPlugin {
	pub fn new() -> impl CommitPlugin {
		Self {}
	}
}

impl CommitPlugin for LogPlugin {
	fn before(&self, _: &Repository) -> Option<Error> {
		println!("log plugin runing.");
		None
	}

	fn after(&self, _: &Repository) -> Option<Error> {
		println!("log plugin runing.");
		None
	}
}
