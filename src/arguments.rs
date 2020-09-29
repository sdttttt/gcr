use clap::{App, Arg, ArgMatches};
use git2::Error;

use crate::metadata::*;

pub struct Arguments {
    mode: Mode,
    params: String,
}

impl Arguments {
    pub fn collect() -> Result<Self, Error> {
        let matches = Self::cli().get_matches();

        Self::resolve_command(matches)
    }

    pub fn new(mode: Mode, params: &str) -> Self {
        let params = String::from(params);
        Self { mode, params }
    }

    pub fn command_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn files(&self) -> &str {
        &self.params.as_str()
    }

    fn cli() -> App<'static, 'static> {
        App::new(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESCRIPTION)
            .args(&[Self::push_arg(PUSH_COMMAND), Self::add_arg(ADD_COMMAND)])
    }

    fn push_arg(command_name: &str) -> Arg {
        Arg::with_name(command_name)
            .short(PUSH_COMMAND_SHORT)
            .long(command_name)
            .required(false)
            .help(PUSH_COMMAND_HELP)
            .takes_value(true)
    }

    fn add_arg(command_name: &str) -> Arg {
        Arg::with_name(command_name)
            .short(ADD_COMMAND_SHORT)
            .long(command_name)
            .required(false)
            .help(ADD_COMMAND_HELP)
            .takes_value(true)
    }

    fn resolve_command(matches: ArgMatches) -> Result<Self, Error> {
        let arg: Self;
        if matches.is_present(ADD_COMMAND) {
            if let Some(files) = matches.value_of(ADD_COMMAND) {
                if files == "." {
                    arg = Self::new(Mode::AddAll, files);
                } else {
                    arg = Self::new(Mode::Add, files);
                }
            } else {
                return Err(Error::from_str(ADD_COMMAND_NO_FILE));
            }
        } else if matches.is_present(PUSH_COMMAND) {
            if let Some(files) = matches.value_of(PUSH_COMMAND) {
                if files == "." {
                    arg = Self::new(Mode::Auto, files);
                } else {
                    arg = Self::new(Mode::Push, files);
                }
            } else {
                return Err(Error::from_str(PUSH_COMMAND_NO_FILE));
            }
        } else {
            arg = Self::new(Mode::Commit, "");
        }
        Ok(arg)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn quick_command_run(vec: Vec<&str>) -> Arguments {
        let matches = Arguments::cli().get_matches_from(vec);

        Arguments::resolve_command(matches).unwrap()
    }

    #[test]
    fn add_all_mode() {
        let args = quick_command_run(vec!["grc", "--add", "."]);
        match args.command_mode() {
            Mode::AddAll => {}
            _ => panic!("NOT ADDALL MODE!"),
        }
    }

    #[test]
    fn add_mode() {
        let args = quick_command_run(vec!["grc", "--add", "rusty"]);
        match args.command_mode() {
            Mode::Add => {}
            _ => panic!("NOT ADD MODE!"),
        }
    }

    #[test]
    fn push_mode() {
        let args = quick_command_run(vec!["grc", "--push", "ytsur"]);
        match args.command_mode() {
            Mode::Push => {}
            _ => panic!("NOT PUSH MODE!"),
        }
    }

    #[test]
    fn auto_mode() {
        let args = quick_command_run(vec!["grc", "--push", "."]);
        match args.command_mode() {
            Mode::Auto => {}
            _ => panic!("NOT AUTO MODE!"),
        }
    }

    #[test]
    fn commit_mode() {
        let args = quick_command_run(vec!["grc"]);
        match args.command_mode() {
            Mode::Commit => {}
            _ => panic!("NOT COMMIT MODE!"),
        }
    }
}
