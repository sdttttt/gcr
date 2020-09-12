use std::process::Command;

pub struct GExector {
    message: String,
    scope: String,
    typ: String
}

impl GExector {
    pub fn new(message: String, scope: String, typ: String) -> GExector {
        GExector {
            message, scope, typ
        }
    }

    fn build_commit_message(&self) -> String {
        if &self.scope.len() != &0 {
            format!("{}:{}", self.typ, self.message)
        } else {
            format!("{}({}):{}", self.typ, self.scope, self.message)
        }
    }

    pub fn commit(&self) {
        let commit_message = self.build_commit_message();       
        let command = format!("git commit -m \"{}\"", commit_message);
        println!("{}", command);
        // Command::new(command).output().expect("failed to execute process");
    }
}