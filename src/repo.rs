use git2::{
    Commit, Error, Index, IndexAddOption, ObjectType, Repository as GRepository, Signature,
    StatusOptions, Statuses,
};

use crate::{
    arguments::Arguments,
    log::{grc_err_println, grc_warn_println, grc_println},
    metadata::Mode,
    util::{author_sign_from_env, committer_sign_from_env, is_all_workspace},
};

/// Repository in GRC.
/// is git2::Repository Encapsulation.
pub struct Repository {
    repo: git2::Repository,
    arg: Arguments,
}

impl Repository {
    pub fn new(path: String, arg: Arguments) -> Result<Self, Error> {
        let result = GRepository::open(&path);
        match result {
            Ok(repo) => Ok(Self { repo, arg }),
            Err(e) => Err(e),
        }
    }

    /// actions before commit.
    pub fn pre_commit(&self) -> Result<(), Error> {
        match self.arg.command_mode() {
            Mode::Commit => self.check_index()?,
            Mode::Add => self.add_files(self.arg.files())?,
            Mode::Auto => {}
            Mode::AddAll => self.add_all_files()?,
            Mode::Push => {}
        };

        Ok(())
    }

    /// actions after commit.
    pub fn after_commit(&self) -> Result<(), Error> {
        match self.arg.command_mode() {
            Mode::Commit => {}
            Mode::Add => {}
            Mode::Auto => {}
            Mode::AddAll => {}
            Mode::Push => {}
        };

        Ok(())
    }

    /// execute git commit.
    pub fn commit(&self, message: &str) -> Result<(), Error> {
        let tree_id = {
            let mut index = self.repo.index()?;
            index.write_tree()?
        };

        let tree = self.repo.find_tree(tree_id)?;
        let commit = self.find_last_commit()?;

		let (author_sign, committer_sign) = self.generate_sign()?;

        self.repo.commit(
            Some("HEAD"),
            &author_sign,
            &committer_sign,
            message,
            &tree,
            &[&commit],
        )?;

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
	/// Priority is given to reading the information of author and committer from env 
	/// and if it does not exist
	/// the user information that has been set up in repo is used. 
	/// Otherwise, Error.
    fn generate_sign(&self) -> Result<(Signature<'static>, Signature<'static>), Error> {

		let mut use_env= false;

		let author_sign = match author_sign_from_env() {
			Ok(sign) => {
				use_env = true;
				sign
			},
			Err(_) => self.repo.signature()?
		};

		let committer_sign = match committer_sign_from_env() {
			Ok(sign) => {
				use_env = true;
				sign
			},
			Err(_) => self.repo.signature()?
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::Mode;
    use crate::util::current_path;
    use crate::Arguments;

    #[test]
    fn test_new_repo() {
        let path = current_path();
        let args = Arguments::new(Mode::Commit, vec![]);
        if let Err(e) = Repository::new(path, args) {
            panic!(e)
        }
    }
}
