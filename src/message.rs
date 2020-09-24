use std::io;

pub struct Messager {
    typ: String,
    scope: String,
    content: String,
}

impl Messager {
    pub fn new() -> Self {
        show_type();
        let typ = ask_type();
        let scope = ask_scope();
        let content = ask_content();
        Self {typ, scope, content}
    }

    pub fn build(&self) -> String {
        if self.scope.len() == 0 || self.scope.trim() == "" {
            format!("{}: {}", self.typ, self.content)
        } else {
            format!("{}({}): {}", self.typ, self.scope, self.content)
        }
    }
}

fn show_type() {
    println!();
    println!("test:       Adding missing tests");
    println!("feat:       A new feature");
    println!("fix:        A bug fix");
    println!("chore:      Build process or auxiliary tool changes");
    println!("docs:       Documentation only changes");
    println!("refactor:   A code change that neither fixes a bug or adds a feature");
    println!("style:      Markup, white-space, formatting, missing semi-colons...");
    println!("perf:       A code change that improves performance");
    println!("ci:         CI related changes");
    println!();
}

fn ask_type() -> String {
   let typ = ask_user("GCR: Message Type ?");
   if typ.len() == 0 || typ.trim() == "" {
      panic!("ooo! type is null !");
   }
   typ
}

fn ask_scope() -> String {
    ask_user("GCR: Scope ? (Optional)")
}

fn ask_content() -> String {
    let content = ask_user("GCR: Commit Message ?");
    if content.len() == 0 || content.trim() == "" {
        panic!("ooo! Message is null!");
    }
    content
}


#[cfg(target_os = "linux")]
pub fn ask_user(question: &str) -> String {
    println!("{}", question);

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("what error?");

    answer.pop();
    answer
}

#[cfg(not(target_os = "linux"))]
pub fn ask_user(question: &str) -> String {
    println!("{}", question);

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("what error?");

    answer.pop();
    answer.pop();
    answer
}

