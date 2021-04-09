use std::rc::Rc;

use git2::{
	Commit, Error, Index, IndexAddOption, ObjectType, Repository as GRepository, Signature,
	StatusOptions, Statuses,
};

use crate::{
	config::Configuration,
	log::{grc_println, grc_warn_println},
	metadata::Mode,
	util::{author_sign_from_env, committer_sign_from_env, is_all_workspace},
};

/// Repository in GRC.
/// is git2::Repository Encapsulation.
pub struct Repository {
	repo: GRepository,
	config: Rc<Configuration>,
}

impl Repository {
	pub fn new(path: String, config: Rc<Configuration>) -> Result<Self, Error> {
		let result = GRepository::open(&path);
		match result {
			Ok(repo) => Ok(Self { repo, config }),
			Err(e) => Err(e),
		}
	}

	pub fn real_repo(&self) -> &GRepository {
		&self.repo
	}

	/// actions before commit.
	/// execute git commit.
	pub fn commit(&self, message: &str) -> Result<(), Error> {
		self.pre_commit()?;

		let tree_id = {
			let mut index = self.repo.index()?;
			index.write_tree()?
		};

		let tree = self.repo.find_tree(tree_id)?;

		let (author_sign, committer_sign) = self.generate_sign()?;

		match self.find_last_commit() {
			Ok(commit) => {
				self.repo.commit(
					Some("HEAD"),
					&author_sign,
					&committer_sign,
					message,
					&tree,
					&[&commit],
				)?;
			}
			Err(_) => {
				grc_warn_println("grc think this is the repo's first commit.");
				self.repo.commit(
					Some("HEAD"),
					&author_sign,
					&committer_sign,
					message,
					&tree,
					&[],
				)?;
			}
		};

		self.after_commit()?;

		Ok(())
	}

	fn pre_commit(&self) -> Result<(), Error> {
		match self.config.command_mode() {
			Mode::Commit => self.check_index()?,
			Mode::Add => self.add_files(self.config.files())?,
			Mode::AddAll => self.add_all_files()?,
		};

		for mut command in self.config.pre_command() {
			if let Err(err_out) = command.spawn() {
				return Err(git2::Error::from_str(
					format!("per command error: {}", err_out.to_string()).as_str(),
				));
			}
		}

		for plug in self.config.plugins() {
			plug.before(&self)?;
		}

		Ok(())
	}

	/// actions after commit.
	fn after_commit(&self) -> Result<(), Error> {
		match self.config.command_mode() {
			Mode::Commit => {}
			Mode::Add => {}
			Mode::AddAll => {}
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
			Some(sign) => {
				use_env = true;
				sign
			}
			None => self.repo.signature()?,
		};

		let committer_sign = match committer_sign_from_env() {
			Some(sign) => {
				use_env = true;
				sign
			}
			None => self.repo.signature()?,
		};

		if use_env {
			grc_println("you are using environment variables to generate commit sign.");
		}

		Ok((author_sign, committer_sign))
	}

	/// the last commit in this repository.
	fn find_last_commit(&self) -> Result<Commit, Error> {
		let obj = self.repo.head()?.resolve()?.peel(ObjectType::Commit)?;
		obj.into_commit().map_err(|_| Error::from_str("not fonund Commit."))
	}

	/// Check to see if the repository commit index is empty.
	fn check_index(&self) -> Result<(), Error> {
		match self.status() {
			Ok(statuses) => {
				let tip = is_all_workspace(&statuses);
				if tip {
					Ok(())
				} else {
					Err(Error::from_str("No files commit to the index."))
				}
			}
			Err(e) => Err(e),
		}
	}
}
