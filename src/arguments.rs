use clap::{App, Arg};

const VERSION: &str = "0.6.0";
const AUTHOR: &str = "SDTTTTT. <sdttttt@outlook.com>";
const NAME: &str = "GRC";
const DESCRIPTION: &str =
    "I'm here to help you make it more standardized and convenient to use Git.";

enum Mode {
    Auto,
    Add,
    Commit,
}

pub struct Arguments {
    mode: Mode,
}

impl Arguments {
    pub fn collect() -> Self {
        let add = "all";
        let auto = "auto";

        let matches = App::new(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESCRIPTION)
            .arg(
                Arg::with_name(add)
                    .short("a")
                    .long("all")
                    .required(false)
                    .help("Help you run `git add .`"),
            )
            .get_matches();

        if matches.is_present(add) {
            Self { mode: Mode::Add }
        } else if matches.is_present(auto) {
            Self { mode: Mode::Auto }
        } else {
            Self { mode: Mode::Commit }
        }
    }
}
