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
	fn before(&self, _: &Repository) -> Result<(), Error> {
		println!("log plugin runing.");
		Ok(())
	}

	fn after(&self, _: &Repository) -> Result<(), Error> {
		println!("log plugin runing.");
		Ok(())
	}

	fn name(&self) -> &str {
		"log"
	}
}
