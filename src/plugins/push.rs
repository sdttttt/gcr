use crate::{log::grc_warn_println, repo::Repository};
use git2::{Cred, Error, PushOptions, RemoteCallbacks};
use std::env;
use std::path::Path;

use super::CommitPlugin;

#[derive(Clone, Copy)]
pub struct PushPlugin;

impl PushPlugin {
	pub fn new() -> impl CommitPlugin {
		Self {}
	}
}

impl CommitPlugin for PushPlugin {
	fn after(&self, repo: &Repository) -> Result<(), Error> {
		println!("[*] running push ...");
		let real_repo = repo.real_repo();

		let head = real_repo.head().unwrap();
		let branch_name = head.shorthand().unwrap_or_else(|| "");
		let config = real_repo.config()?;
		let remote_name = config.get_str(format!("branch.{}.remote", branch_name).as_str())?;

		let mut remote = real_repo.find_remote(remote_name)?;
		let mut callbacks = RemoteCallbacks::new();
		callbacks.credentials(|_, username_from_url, _| {
			Cred::ssh_key(
				username_from_url.unwrap(),
				None,
				Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
				None,
			)
		});

		let mut push_option: PushOptions = PushOptions::new();
		push_option.remote_callbacks(callbacks);

		grc_warn_println(format!("Remote: {}", remote_name));
		grc_warn_println(format!("Branch: {}", branch_name));

		remote.push(
			&[format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name).as_str()],
			Some(&mut push_option),
		)?;
		Ok(())
	}
}
