use git2::{
    Commit, Error, ObjectType, Repository as GRepository, Signature, StatusOptions, Statuses,
};

use crate::{arguments::Arguments, metadata::Mode, util::is_all_workspace};

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

    pub fn default(path: String) -> Result<Self, Error> {
        let result = GRepository::open(&path);
        let arg = Arguments::new(Mode::Commit, "");
        match result {
            Ok(repo) => Ok(Self { repo, arg }),
            Err(e) => Err(e),
        }
    }

    pub fn pre_commit(&self) -> Result<(), Error> {
        match self.arg.command_mode() {
            Mode::Commit => self.check_index()?,
            Mode::Add => {}
            Mode::Auto => {}
            Mode::AddAll => {}
            Mode::Push => {}
        };

        Ok(())
    }

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

    pub fn commit(&self, message: &str) -> Result<(), Error> {
        let current_sign = self.generate_sign();
        let tree_id = {
            let mut index = self.repo.index()?;
            index.write_tree()?
        };

        let tree = self.repo.find_tree(tree_id)?;
        let commit = self.find_last_commit()?;

        self.repo.commit(
            Some("HEAD"),
            &current_sign,
            &current_sign,
            message,
            &tree,
            &[&commit],
        )?;

        Ok(())
    }

    fn status(&self) -> Result<Statuses<'_>, Error> {
        let mut sp = StatusOptions::new();
        self.repo.statuses(Option::from(&mut sp))
    }

    fn generate_sign(&self) -> Signature<'static> {
        self.repo.signature().unwrap()
    }

    fn find_last_commit(&self) -> Result<Commit, Error> {
        let obj = self.repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit()
            .map_err(|_| Error::from_str("not fonund Commit."))
    }

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
    use crate::util::current_path;

    #[test]
    fn test_new_repo() {
        let path = current_path();
        if let Err(e) = Repository::default(path) {
            panic!(e)
        }
    }
}
