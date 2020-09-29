use clap::{App, Arg};

const VERSION: &str = "0.6.0";
const AUTHOR: &str = "SDTTTTT. <sdttttt@outlook.com>";
const NAME: &str = "GRC";
const DESCRIPTION: &str =
    "I'm here to help you make it more standardized and convenient to use Git.";

pub enum Mode {
    Auto,
    Add,
    AddAll,
    Commit,
    Push,
}

const ADD_COMMAND: &str = "add";
const PUSH_COMMAND: &str = "push";

pub struct Arguments {
    mode: Mode,
}

impl Arguments {
    pub fn collect() -> Self {
        let matches = App::new(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESCRIPTION)
            .args(
                &[Self::push_arg(PUSH_COMMAND), Self::add_arg(ADD_COMMAND)]
            )
            .get_matches();

        if matches.is_present(ADD_COMMAND) {
            Self::new(Mode::Add)
        } else if matches.is_present(PUSH_COMMAND) {
            Self::new(Mode::Auto)
        } else {
            Self::new(Mode::Commit)
        }
    }

    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }

    pub fn command_mode(&self) -> &Mode {
        &self.mode
    }

    fn push_arg(command_name: &str) -> Arg {
        Arg::with_name(command_name)
            .short("p")
            .long("push")
            .required(false)
            .help("Help you run `git add .` and `git push`")
    }

    fn add_arg(command_name: &str) -> Arg {
        Arg::with_name(command_name)
            .short("a")
            .long("add")
            .required(false)
            .help("Help you run `git add .`")
    }
}
