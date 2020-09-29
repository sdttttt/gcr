use clap::{App, Arg, ArgMatches};
use git2::Error;

use crate::metadata::*;

pub struct Arguments {
    mode: Mode,
    params: String,
}

impl Arguments {
    pub fn collect() -> Result<Self, Error> {
        let matches = App::new(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESCRIPTION)
            .args(&[Self::push_arg(PUSH_COMMAND), Self::add_arg(ADD_COMMAND)])
            .get_matches();

        let arg = Self::resolve_command(matches)?;
        Ok(arg)
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

    fn push_arg(command_name: &str) -> Arg {
        Arg::with_name(command_name)
            .short(PUSH_COMMAND_SHORT)
            .required(false)
            .help(PUSH_COMMAND_HELP)
            .takes_value(true)
    }

    fn add_arg(command_name: &str) -> Arg {
        Arg::with_name(command_name)
            .short(ADD_COMMAND_SHORT)
            .required(false)
            .help(ADD_COMMAND_HELP)
            .takes_value(true)
    }

    fn resolve_command(matches: ArgMatches) -> Result<Self, Error> {
        let arg: Self;
        if matches.is_present(ADD_COMMAND) {
            if let Some(files) = matches.value_of(ADD_COMMAND) {
                arg = Self::new(Mode::Add, files);
            } else {
                return Err(Error::from_str(ADD_COMMAND_NO_FILE));
            }
        } else if matches.is_present(PUSH_COMMAND) {
            if let Some(files) = matches.value_of(PUSH_COMMAND) {
                arg = Self::new(Mode::Push, files);
            } else {
                return Err(Error::from_str(PUSH_COMMAND_NO_FILE));
            }
        } else {
            arg = Self::new(Mode::Commit, "");
        }
        Ok(arg)
    }
}
