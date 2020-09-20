use git2::{Repository as GRepository, Signature, Error};


pub struct Repository {
    repo: git2::Repository
}

impl Repository {
    pub fn new(path: String) -> Self {
        let result = GRepository::open(path);

        match result {
            Ok(repo) => Self { repo },
            Err(e) => panic!(e)
        }
    }

    pub fn commit(&self, message: &str) -> Result<(), Error> {
        let current_sign = self.generate_sign();
        let tree_id = {
            let mut index = self.repo.index()?;
            
            index.write_tree()?
        };

        let tree = self.repo.find_tree(tree_id)?;

        self.repo.commit(
            Some("HEAD"),
            &current_sign,
            &current_sign,
            message,
            &tree,
            &[])?;

        Ok(())
    }

    fn generate_sign(&self) -> Signature<'static> {
        self.repo.signature().unwrap()
    }
}
