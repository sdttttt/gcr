use crate::repo::Repository;
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
		println!("[+] running push ...");
		let real_repo = repo.real_repo();

		let head = real_repo.head()?;
		let head = head.shorthand().unwrap_or_else(|| "");

		let mut remote = real_repo.find_remote("origin")?;
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

		remote.push(
			&[format!("refs/heads/{}:refs/heads/{}", head, head).as_str()],
			Some(&mut push_option),
		)?;
		Ok(())
	}
}
