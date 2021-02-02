use std::rc::Rc;

use crate::{arguments::Arguments, extensions::Extensions};
// Both command-line arguments and configuration files can specify configuration
// options, and the two are combined here.
pub struct Configuration {
	extends_type: Vec<String>,
	params: Vec<String>,
	emoji: bool,
}

impl Configuration {
	pub fn merge(arg: Arguments, ext: Extensions) -> Rc<Self> {

		let params = arg.files().clone();
		let extends_type = ext.types().clone();

		let emoji = {
			if ext.emoji() || arg.emoji() {
				ext.emoji()
			} else {
				false
			}
		};

		Rc::new(Self { params, extends_type,emoji })
	}

	pub fn extends_type(&self) -> &Vec<String> {
		&self.extends_type
	}

	pub fn files(&self) ->  &Vec<String> {
		&self.params
	}

	pub fn emoji(&self) -> bool {
		self.emoji
	}
}