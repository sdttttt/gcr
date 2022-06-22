use std::rc::Rc;
use std::{
	path::{Path, PathBuf},
	process::Command,
};

use git2::{
	Commit, Error, Index, IndexAddOption, ObjectType, Repository as GRepository, Signature,
	StatusOptions, Statuses,
};

use crate::log::grc_success_println;
use crate::version::VERSION;
use crate::{
	config::Configuration,
	log::{grc_println, grc_warn_println},
	metadata::Mode,
	util::{author_sign_from_env, committer_sign_from_env, get_tracked_files},
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

	#[cfg(feature = "plug")]
	pub fn real_repo(&self) -> &GRepository {
		&self.repo
	}

	/// actions before commit.
	/// execute git commit.
	pub fn commit(&self, message: &str) -> Result<(), Error> {
		self.pre_commit()?;

		if self.config.command_mode() == &Mode::Version {
			grc_success_println(format!("GRC {}", VERSION));
			return Ok(());
		}

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
		let tracked = match self.config.command_mode() {
			Mode::Commit => self.check_index()?,
			_ => Vec::new(),
		};

		self.execute_hook_command(self.config.pre_command())?;

		#[cfg(feature = "plug")]
		for plug in self.config.plugins() {
			plug.before(&self)?;
		}

		match self.config.command_mode() {
			Mode::Commit => self.add_files(&tracked)?,
			Mode::Add => self.add_files(self.config.files())?,
			Mode::AddAll => self.add_all_files()?,
			_ => (),
		};

		Ok(())
	}

	/// actions after commit.
	fn after_commit(&self) -> Result<(), Error> {
		match self.config.command_mode() {
			Mode::Commit => {}
			Mode::Add => {}
			Mode::AddAll => {}
			_ => (),
		};

		self.execute_hook_command(self.config.after_command())?;

		#[cfg(feature = "plug")]
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
	fn add_files<'a, P: AsRef<Path>>(&self, files_path: &'a Vec<P>) -> Result<(), Error> {
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

	fn check_index(&self) -> Result<Vec<PathBuf>, Error> {
		match self.status() {
			Ok(statuses) => {
				let tracked = get_tracked_files(&statuses);
				if tracked.len() != 0 {
					Ok(tracked)
				} else {
					Err(Error::from_str("No files commit to the index."))
				}
			}
			Err(e) => Err(e),
		}
	}

	fn execute_hook_command(&self, commands: Vec<Command>) -> Result<(), Error> {
		for mut command in commands {
			match command.output() {
				Ok(out) => {
					grc_println(String::from_utf8_lossy(&out.stdout));
					if !out.status.success() {
						return Err(Error::from_str("hook command error"));
					}
				}

				Err(out) => {
					return Err(Error::from_str(
						format!("hook command error: {}", out.to_string()).as_str(),
					));
				}
			}
		}

		Ok(())
	}
}
