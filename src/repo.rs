use std::rc::Rc;

use git2::{
	Commit, Error, Index, IndexAddOption, ObjectType, Repository as GRepository, Signature,
	StatusOptions, Statuses,
};

use crate::{
	config::Configuration,
	log::{grc_log, grc_warn},
	metadata::Mode,
	util::{author_sign_from_env, committer_sign_from_env, is_all_workspace, repo_gpg_available},
};

/// Repository in GRC.
/// is git2::Repository Encapsulation.
pub struct Repository {
	repo:   GRepository,
	config: Rc<Configuration>,
}

impl Repository {
	pub fn new(path: String, config: Rc<Configuration>) -> Result<Self, Error> {
		let result = GRepository::open(&path);
		match result {
			| Ok(repo) => Ok(Self { repo, config }),
			| Err(e) => Err(e),
		}
	}

	pub fn real_repo(&self) -> &GRepository {
		&self.repo
	}

	/// actions before commit.
	/// execute git commit.
	pub fn commit(&self, message: &str) -> Result<(), Error> {
		self.pre_commit()?;

		let is_gpg_commit = self.gpg_sign_commit(message)?;
		if !is_gpg_commit {
			self.general_commit(message)?;
		}

		self.after_commit()?;

		Ok(())
	}

	fn general_commit(&self, message: &str) -> Result<(), Error> {
		let tree_id = {
			let mut index = self.repo.index()?;
			index.write_tree()?
		};

		let tree = self.repo.find_tree(tree_id)?;
		let (author_sign, committer_sign) = self.generate_sign()?;

		match self.find_last_commit() {
			| Ok(commit) => {
				self.repo.commit(
					Some("HEAD"),
					&author_sign,
					&committer_sign,
					message,
					&tree,
					&[&commit],
				)?;
			}
			| Err(_) => {
				grc_warn("grc think this is the repo's first commit.");
				self.repo.commit(
					Some("HEAD"),
					&author_sign,
					&committer_sign,
					message,
					&tree,
					&[],
				)?;
			}
		}
		Ok(())
	}

	/// check gpg is available, if true then using gpg sign commit.
	/// github.com/sdttttt/gcr/issues/52 Thinks @Enter-tainer @CoelacanthusHex
	fn gpg_sign_commit(&self, message: &str) -> Result<bool, Error> {
		match repo_gpg_available(&self.repo) {
			| Some(ref key) => {
				let tree_id = {
					let mut index = self.repo.index()?;
					index.write_tree()?
				};

				let tree = self.repo.find_tree(tree_id)?;
				let (author_sign, committer_sign) = self.generate_sign()?;

				let buf = match self.find_last_commit() {
					| Ok(commit) => self.repo.commit_create_buffer(
						&author_sign,
						&committer_sign,
						message,
						&tree,
						&[&commit],
					)?,
					| Err(_) => {
						grc_warn("grc think this is the repo's first commit.");
						self.repo.commit_create_buffer(
							&author_sign,
							&committer_sign,
							message,
							&tree,
							&[],
						)?
					}
				};

				// https://github.com/rust-lang/git2-rs/issues/507 Thinks @cole-h
				let commit_content = std::str::from_utf8(&buf).unwrap();
				let ret = self.repo.commit_signed(commit_content, key, Some(key))?;
				let commit = self.repo.find_commit(ret)?;
				self.repo.branch(&self.current_branch_name(), &commit, false)?;

				Ok(true)
			}

			| None => Ok(false),
		}
	}

	fn pre_commit(&self) -> Result<(), Error> {
		match self.config.command_mode() {
			| Mode::Commit => self.check_index()?,
			| Mode::Add => self.add_files(self.config.files())?,
			| Mode::AddAll => self.add_all_files()?,
		};

		for plug in self.config.plugins() {
			plug.before(&self)?;
		}

		Ok(())
	}

	/// actions after commit.
	fn after_commit(&self) -> Result<(), Error> {
		match self.config.command_mode() {
			| Mode::Commit => {}
			| Mode::Add => {}
			| Mode::AddAll => {}
		};

		for plug in self.config.plugins() {
			plug.after(&self)?;
		}

		Ok(())
	}

	/// Repository status.
	fn status(&self) -> Result<Statuses<'_>, Error> {
		let mut sp = StatusOptions::new();
		self.repo.statuses(Option::from(&mut sp))
	}

	/// Repository commit index.
	fn index(&self) -> Result<Index, Error> {
		self.repo.index()
	}

	/// add files to Repository commit index.
	fn add_files(&self, files_path: &Vec<String>) -> Result<(), Error> {
		let mut index = self.index()?;
		for file_path in files_path {
			index.add_path(file_path.as_ref())?;
		}
		index.write()?;
		Ok(())
	}

	/// add all files to Repository commit index.
	fn add_all_files(&self) -> Result<(), Error> {
		let mut index = self.index()?;
		index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
		index.write()?;
		Ok(())
	}

	/// generate commit sign
	/// Priority is given to reading the information of author and committer
	/// from env and if it does not exist
	/// the user information that has been set up in repo is used.
	/// Otherwise, Error.
	fn generate_sign(&self) -> Result<(Signature<'static>, Signature<'static>), Error> {
		let mut use_env = false;

		let author_sign = match author_sign_from_env() {
			| Some(sign) => {
				use_env = true;
				sign
			}
			| None => self.repo.signature()?,
		};

		let committer_sign = match committer_sign_from_env() {
			| Some(sign) => {
				use_env = true;
				sign
			}
			| None => self.repo.signature()?,
		};

		if use_env {
			grc_log("you are using environment variables to generate commit sign.");
		}

		Ok((author_sign, committer_sign))
	}

	/// the last commit in this repository.
	fn find_last_commit(&self) -> Result<Commit, Error> {
		let obj = self.repo.head()?.resolve()?.peel(ObjectType::Commit)?;
		obj.into_commit().map_err(|_| Error::from_str("not fonund Commit."))
	}

	fn current_branch_name(&self) -> String {
		let head = &self.repo.head().unwrap();
		let branch_name = head.shorthand().unwrap_or_else(|| "");
		String::from(branch_name)
	}

	/// Check to see if the repository commit index is empty.
	fn check_index(&self) -> Result<(), Error> {
		match self.status() {
			| Ok(statuses) => {
				let tip = is_all_workspace(&statuses);
				if tip {
					Ok(())
				} else {
					Err(Error::from_str("No files commit to the index."))
				}
			}
			| Err(e) => Err(e),
		}
	}
}

//#[cfg(test)]
//mod tests {

//	use git2::Repository;

//	#[test]
//	fn check_gpg() {
//		let repo = Repository::open(".").unwrap();
//		let config = repo.config().unwrap();
//		let gpg_enabled = config.get_bool("commit.gpgsign").unwrap_or_else(|_|
// false); 		assert_eq!(gpg_enabled, true); 	}
//}
