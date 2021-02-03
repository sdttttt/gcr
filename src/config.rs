use std::rc::Rc;

use crate::{arguments::Arguments, extensions::Extensions, metadata::Mode};

// Both command-line arguments and configuration files can specify configuration
// options, and the two are combined here.
pub struct Configuration {
	mode:            Mode,
	extends_type:    Vec<String>,
	params:          Vec<String>,
	overwrite_emoji: Vec<String>,
	emoji:           bool,
}

impl Configuration {
	pub fn merge(arg: Arguments, ext: Extensions) -> Rc<Self> {
		let params = arg.files().clone();
		let extends_type = ext.types().unwrap_or(&vec![]).clone();
		let mode = arg.command_mode();
		let emoji = ext.emoji() || arg.emoji();
		let overwrite_emoji =
			if emoji { ext.overwrite_emoji().unwrap_or(&vec![]).clone() } else { vec![] };

		Rc::new(Self { params, extends_type, emoji, mode, overwrite_emoji })
	}

	//#[cfg(test)]
	//pub fn default() -> Rc<Self> {
	//	let ext = Extensions::from_agreement().unwrap();
	//	let arg = Arguments::default();

	//	Self::merge(arg, ext)
	//}

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
}
