use clap::{App, Arg, ArgMatches};
use git2::Error;

use crate::metadata::*;
use crate::util::*;

/// Parse the behavior and extra parameters of GRC by entering commands.
pub struct Arguments {
	mode:            Mode,
	params:          Vec<String>,
	config_filename: String,
}

// Get the external parameter and analyze it. Construct the behavior of GRC.
impl Arguments {
	// get the external parameter.
	pub fn collect() -> Result<Self, Error> {
		let matches = Self::cli().get_matches();
		Self::resolve_command(matches)
	}

	pub fn command_mode(&self) -> &Mode {
		&self.mode
	}

	pub fn files(&self) -> &Vec<String> {
		&self.params
	}

	fn default() -> Self {
		Self {
			mode:            Mode::Commit,
			params:          vec![],
			config_filename: String::new(),
		}
	}

	fn cli() -> App<'static, 'static> {
		App::new(NAME)
			.version(VERSION)
			.author(AUTHOR)
			.about(DESCRIPTION)
			.args(&[Self::add_arg(), Self::designate_config_arg()])
	}

	fn add_arg() -> Arg<'static, 'static> {
		Arg::with_name(ADD_PARAMS)
			.short(ADD_COMMAND_SHORT)
			.long(ADD_COMMAND)
			.multiple(true)
			.required(false)
			.help(ADD_COMMAND_HELP)
			.takes_value(true)
	}

	fn designate_config_arg() -> Arg<'static, 'static> {
		Arg::with_name(DESIGNATE_CONFIG_PARAMS)
			.short(DESIGNATE_CONFIG_COMMAND_SHORT)
			.long(DESIGNATE_CONFIG_COMMAND)
			.required(false)
			.help(DESIGNATE_CONFIG_COMMAND_HELP)
			.takes_value(true)
	}

	/// Construct the behavior according to the input parameters.
	fn resolve_command(matches: ArgMatches) -> Result<Self, Error> {
		let mut arg = Self::default();

		match Self::finally_entry_handle(&mut arg, &matches) {
			| Ok(_) => Ok(arg),
			| Err(e) => Err(e),
		}
	}

	// fn extend_handle() {
	// 	// extend-handle: fn(&mut Arguments, &ArgMatches) -> Result<bool, Error>
	// }

	fn finally_entry_handle(arg: &mut Arguments, matches: &ArgMatches) -> Result<bool, Error> {
		// finally-handle: fn(&mut Arguments, &ArgMatches) -> Result<bool, Error>
		// that confirm the behavior to be used by the GRC and the required parameters.
		let post_handlers = &[Self::add_params_handle];

		for handle in post_handlers {
			match handle(arg, &matches) {
				| Ok(true) => return Ok(true),
				| Ok(false) => {}
				| Err(e) => return Err(e),
			}
		}

		Ok(false)
	}

	fn add_params_handle(arg: &mut Arguments, matches: &ArgMatches) -> Result<bool, Error> {
		if matches.is_present(ADD_PARAMS) {
			if let Some(files) = matches.values_of(ADD_PARAMS) {
				let files_vec: Vec<String> = vec_str_to_string(files.collect());
				if files_vec.len() == 1 && files_vec[0] == "." {
					arg.mode = Mode::AddAll;
				} else {
					arg.mode = Mode::Add;
					arg.params = files_vec;
				}
				return Ok(true);
			} else {
				return Err(Error::from_str(ADD_COMMAND_NO_FILE));
			}
		}
		Ok(false)
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
	fn add_all_mode() -> Result<(), &'static str> {
		let args = quick_command_run(vec!["grc", "--add", "."]);
		match args.command_mode() {
			| Mode::AddAll => Ok(()),
			| _ => Err("NOT ADDALL MODE!"),
		}
	}

	#[test]
	fn add_mode() {
		let args = quick_command_run(vec!["grc", "--add", "rusty"]);
		match args.command_mode() {
			| Mode::Add => {}
			| _ => panic!("NOT ADD MODE!"),
		}
	}

	#[test]
	fn commit_mode() {
		let args = quick_command_run(vec!["grc"]);
		match args.command_mode() {
			| Mode::Commit => {}
			| _ => panic!("NOT COMMIT MODE!"),
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
