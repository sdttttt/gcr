use git2::Repository as GRepository;

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

    pub fn commit(&self, message: String) {

    }

    pub fn sign(&self) {
        let sign = self.repo.signature().unwrap();
        let email = sign.email().unwrap();
        let username = sign.name().unwrap();
    
        println!("{}", email);
        println!("{}", username);
    }
}
