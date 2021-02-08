	use git2::{Error, Direction};

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

	fn after(&self, repo: &Repository) ->  Result<(), Error> {
		println!("[+] running push ...");
		let real_repo = repo.real_repo();
		let mut remote = real_repo.find_remote("origin")?;
		remote.connect(Direction::Push)?;
		remote.push(&[""], None)?;
		Ok(())
	}
}
