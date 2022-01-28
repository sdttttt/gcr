use std::{process::Command, rc::Rc};

use crate::{
	arguments::Arguments,
	extensions::Extensions,
	metadata::Mode,
	plugins::{find_plug, CommitPlugin},
	util::parse_command,
};

// Both command-line arguments and configuration files can specify configuration
// options, and the two are combined here.
pub struct Configuration {
	mode: Mode,
	extends_type: Vec<String>,
	params: Vec<String>,
	overwrite_emoji: Vec<String>,
	plugs: Vec<Rc<dyn CommitPlugin>>,
	pre_command: Vec<String>,
	after_command: Vec<String>,
	emoji: bool,
}

impl Configuration {
	pub fn merge(arg: Arguments, ext: Extensions) -> Rc<Self> {
		let params = arg.files().clone();

		let extends_type = ext.types().unwrap_or(&vec![]).clone();

		let mode = arg.command_mode();

		let emoji = ext.emoji() || arg.emoji();

		let pre_command = ext.pre_command().unwrap_or(&vec![]).clone();
		let after_command = ext.after_command().unwrap_or(&vec![]).clone();

		let overwrite_emoji =
			if emoji { ext.overwrite_emoji().unwrap_or(&vec![]).clone() } else { vec![] };

		let plugs = find_plug(ext.plug().unwrap_or(&vec![]));

		Rc::new(Self {
			params,
			extends_type,
			emoji,
			mode,
			overwrite_emoji,
			plugs,
			pre_command,
			after_command,
		})
	}

	#[cfg(test)]
	pub fn default() -> Rc<Self> {
		let ext = Extensions::from_agreement().unwrap();
		let arg = Arguments::default();

		Self::merge(arg, ext)
	}

	pub fn command_mode(&self) -> &Mode {
		&self.mode
	}

	pub fn extends_type(&self) -> &Vec<String> {
		&self.extends_type
	}

	pub fn files(&self) -> &Vec<String> {
		&self.params
	}

	pub fn emoji(&self) -> bool {
		self.emoji
	}

	pub fn overwrite_emoji(&self) -> &Vec<String> {
		&self.overwrite_emoji
	}

	pub fn pre_command(&self) -> Vec<Command> {
		parse_command(&self.pre_command)
	}

	pub fn after_command(&self) -> Vec<Command> {
		parse_command(&self.after_command)
	}

	pub fn plugins(&self) -> &Vec<Rc<dyn CommitPlugin>> {
		&self.plugs
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_merge() {
		let conf = Configuration::default();
		assert!(conf.emoji());
		assert_eq!(conf.files().len(), 0);
		assert_eq!(conf.command_mode(), &Mode::Commit);
		assert_eq!(conf.plugins().len(), 0);
		assert_eq!(conf.overwrite_emoji().len(), 0);
		assert!(conf.extends_type().len() > 0);
		assert_eq!(conf.pre_command().len(), 0);
		assert_eq!(conf.after_command().len(), 0);
	}
}
