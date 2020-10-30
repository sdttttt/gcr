use clap::{App, Arg, ArgMatches};
use git2::Error;

use crate::metadata::*;
use crate::util::*;

pub struct Arguments {
    mode: Mode,
    params: Vec<String>,
}

// Get the external parameter and analyze it. Construct the behavior of GRC.
impl Arguments {
    // get the external parameter.
    pub fn collect() -> Result<Self, Error> {
        let matches = Self::cli().get_matches();

        Self::resolve_command(matches)
    }

    pub fn new(mode: Mode, params: Vec<String>) -> Self {
        Self { mode, params }
    }

    pub fn command_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn files(&self) -> &Vec<String> {
        &self.params
    }

    fn cli() -> App<'static, 'static> {
        App::new(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESCRIPTION)
            .args(&[Self::add_arg(ADD_PARAMS)])
        // .args(&[Self::push_arg(PUSH_PARAMS), Self::add_arg(ADD_PARAMS)])
    }

    fn push_arg(params_name: &str) -> Arg {
        Arg::with_name(params_name)
            .short(PUSH_COMMAND_SHORT)
            .long(PUSH_COMMAND)
            .multiple(true)
            .required(false)
            .help(PUSH_COMMAND_HELP)
            .takes_value(true)
    }

    fn add_arg(params_name: &str) -> Arg {
        Arg::with_name(params_name)
            .short(ADD_COMMAND_SHORT)
            .long(ADD_COMMAND)
            .multiple(true)
            .required(false)
            .help(ADD_COMMAND_HELP)
            .takes_value(true)
    }

    // Construct the behavior according to the input parameters.
    fn resolve_command(matches: ArgMatches) -> Result<Self, Error> {
        let arg: Self;
        if matches.is_present(ADD_PARAMS) {
            if let Some(files) = matches.values_of(ADD_PARAMS) {
                let files_vec: Vec<String> = vec_str_to_string(files.collect());
                if files_vec.len() == 1 && files_vec[0] == "." {
                    arg = Self::new(Mode::AddAll, vec![]);
                } else {
                    arg = Self::new(Mode::Add, files_vec);
                }
            } else {
                return Err(Error::from_str(ADD_COMMAND_NO_FILE));
            }
        } else if matches.is_present(PUSH_PARAMS) {
            if let Some(files) = matches.values_of(PUSH_PARAMS) {
                let files_vec: Vec<String> = vec_str_to_string(files.collect());
                if files_vec.len() == 1 && files_vec[0] == "." {
                    arg = Self::new(Mode::Auto, vec![]);
                } else {
                    arg = Self::new(Mode::Push, files_vec);
                }
            } else {
                return Err(Error::from_str(PUSH_COMMAND_NO_FILE));
            }
        } else {
            arg = Self::new(Mode::Commit, vec![]);
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
    #[ignore]
    fn push_mode() {
        let args = quick_command_run(vec!["grc", "--push", "ytsur"]);
        match args.command_mode() {
            Mode::Push => {}
            _ => panic!("NOT PUSH MODE!"),
        }
    }

    #[test]
    #[ignore]
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

    #[test]
    fn input_file() {
        let file_1 = "1.txt";
        let args = quick_command_run(vec!["grc", "--add", file_1]);
        for file_name in args.files() {
            if file_name.as_str() != file_1 {
                panic!("NOT THIS FILE NAME.")
            }
        }
    }

    #[test]
    fn input_more_file() {
        let file_1 = "1.txt";
        let file_2 = "2.txt";
        let file_3 = "3.txt";

        let args = quick_command_run(vec!["grc", "--add", file_1, file_2, file_3]);
        for file_name in args.files() {
            if file_name.as_str() != file_1
                && file_name.as_str() != file_2
                && file_name.as_str() != file_3
            {
                panic!("NOT THIS FILE NAME.")
            }
        }
    }
}
