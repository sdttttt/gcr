use clap::{Arg, ArgMatches, Command};
use git2::Error;

use crate::metadata::*;
use crate::util::*;

/// Parse the behavior and extra parameters of GRC by entering commands.
pub struct Arguments {
	mode: Mode,
	params: Vec<String>,
	config_filename: String,
	emoji: bool,
}

// Get the external parameter and analyze it. Construct the behavior of GRC.
impl Arguments {
	// get the external parameter.
	pub fn collect() -> Result<Self, Error> {
		let matches = Self::cli().get_matches();
		Self::resolve_command(matches)
	}

	pub fn has_specified_config(&self) -> bool {
		!self.config_filename.is_empty()
	}

	pub fn config_file(&self) -> &str {
		&self.config_filename.as_str()
	}

	pub fn command_mode(&self) -> Mode {
		self.mode.clone()
	}

	pub fn files(&self) -> &Vec<String> {
		&self.params
	}

	pub fn emoji(&self) -> bool {
		self.emoji
	}

	pub fn default() -> Self {
		Self { mode: Mode::Commit, params: vec![], config_filename: String::new(), emoji: false }
	}

	fn cli() -> Command<'static> {
		Command::new(NAME).version(VERSION).author(AUTHOR).about(DESCRIPTION).args(&[
			Self::add_arg(),
			Self::designate_config_arg(),
			Self::emoji_arg(),
			Self::version_arg(),
		])
	}

	fn add_arg() -> Arg<'static> {
		Arg::new(ADD_PARAMS)
			.short(ADD_COMMAND_SHORT)
			.long(ADD_COMMAND)
			.multiple_occurrences(true)
			.multiple_values(true)
			.required(false)
			.help(ADD_COMMAND_HELP)
			.takes_value(true)
	}

	fn designate_config_arg() -> Arg<'static> {
		Arg::new(DESIGNATE_CONFIG_PARAMS)
			.short(DESIGNATE_CONFIG_COMMAND_SHORT)
			.long(DESIGNATE_CONFIG_COMMAND)
			.required(false)
			.help(DESIGNATE_CONFIG_COMMAND_HELP)
			.takes_value(true)
	}

	fn emoji_arg() -> Arg<'static> {
		Arg::new(EMOJI_COMMAND)
			.long(EMOJI_COMMAND)
			.required(false)
			.help(EMOJI_COMMAND_HELP)
			.takes_value(false)
	}

	fn version_arg() -> Arg<'static> {
		Arg::new(VERSION_COMMAND)
			.short(VERSION_COMMAND_SHORT)
			.long(VERSION_COMMAND)
			.required(false)
			.help(VERSION_COMMAND_HELP)
			.takes_value(false)
	}

	/// Construct the behavior according to the input parameters.
	fn resolve_command(matches: ArgMatches) -> Result<Self, Error> {
		let mut arg = Self::default();

		Self::extends_handle_chain(&mut arg, &matches)?;
		Self::finally_handle_chain(&mut arg, &matches)?;

		Ok(arg)
	}

	fn extends_handle_chain(arg: &mut Arguments, matches: &ArgMatches) -> Result<bool, Error> {
		// extend-handle: fn(&mut Arguments, &ArgMatches) -> Result<bool, Error>
		// extended GRC parameters will be handled here. They are processed before the
		// final behavior is determined.
		let before_handles = &[Self::designate_config_handle, Self::emoji_check_handle];

		for handle in before_handles {
			handle(arg, matches)?;
		}

		Ok(true)
	}

	fn designate_config_handle(arg: &mut Arguments, matches: &ArgMatches) -> Result<bool, Error> {
		if matches.is_present(DESIGNATE_CONFIG_PARAMS) {
			if let Some(config_filename) = matches.value_of(DESIGNATE_CONFIG_PARAMS) {
				arg.config_filename = config_filename.to_string();
				return Ok(true);
			}
		}

		Ok(false)
	}

	fn emoji_check_handle(arg: &mut Arguments, matches: &ArgMatches) -> Result<bool, Error> {
		if matches.is_present(EMOJI_COMMAND) {
			arg.emoji = true;
			return Ok(true);
		}
		return Ok(false);
	}

	fn finally_handle_chain(arg: &mut Arguments, matches: &ArgMatches) -> Result<bool, Error> {
		// finally-handle: fn(&mut Arguments, &ArgMatches) -> Result<bool, Error>
		// that confirm the behavior to be finally used by the GRC and the required
		// parameters. This is a chain of responsibility
		let post_handles = &[Self::add_params_handle];

		for handle in post_handles {
			match handle(arg, matches) {
				Ok(true) => return Ok(true),
				Ok(false) => {}
				Err(e) => return Err(e),
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
	fn test_clap3_verify() {
		Arguments::cli().debug_assert();
	}

	#[test]
	fn add_all_mode() {
		let args = quick_command_run(vec!["grc", "--add", "."]);

		assert_eq!(args.command_mode(), Mode::AddAll);
	}

	#[test]
	fn add_mode() {
		let args = quick_command_run(vec!["grc", "--add", "rusty"]);
		assert_eq!(args.command_mode(), Mode::Add);
	}

	#[test]
	fn enable_emoji() {
		let args = quick_command_run(vec!["grc", "--emoji"]);
		assert!(args.emoji())
	}

	#[test]
	fn commit_mode() {
		let args = quick_command_run(vec!["grc"]);
		assert_eq!(args.command_mode(), Mode::Commit);
	}

	#[test]
	fn input_file() {
		let file_1 = "1.txt";
		let args = quick_command_run(vec!["grc", "--add", file_1]);
		for file_name in args.files() {
			assert_eq!(file_name.as_str(), file_1);
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
