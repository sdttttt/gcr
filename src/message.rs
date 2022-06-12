use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use dialoguer::{Confirm, Editor};

use crate::log::grc_err_println;
use crate::metadata::*;
use crate::util::remove_pound_prefix;

/// 0: name 1: description 2: emoji
struct CommitTD(String, String, Option<String>);

/// Messsager is Commit Message struct.
pub struct Messager {
	commit_type_descript: Vec<CommitTD>,

	typ: String,
	emoji: Option<String>,
	scope: String,
	subject: String,
	body: String,
}

impl CommitTD {
	pub fn from(type_name: &str, type_desc: &str) -> Self {
		Self(type_name.to_owned(), type_desc.to_owned(), None)
	}

	pub fn with_emoji(type_name: &str, type_desc: &str, emoji: &str) -> Self {
		Self(type_name.to_owned(), type_desc.to_owned(), Some(emoji.to_owned()))
	}

	pub fn update_emoji(&mut self, emoji: &str) {
		self.2 = Some(emoji.to_owned())
	}
}

impl Messager {
	pub fn new(enable_emoji: bool) -> Self {
		let commit_type_descript = BASE_COMMIT_TYPE_DESCRIPTION
			.iter()
			.map(|td: &(&str, &str)| -> CommitTD {
				let type_name = td.0.to_owned();
				let type_desc = td.1.to_owned();
				let mut emoji = String::new();

				if enable_emoji {
					for emj in BASE_COMMIT_TYPE_EMOJI {
						if emj.0 == td.0 {
							emoji = emj.1.to_owned();
							break;
						}
					}
				}

				CommitTD::with_emoji(type_name.as_str(), type_desc.as_str(), emoji.as_str())
			})
			.collect();
		let typ = String::new();
		let scope = String::new();
		let subject = String::new();
		let body = String::new();

		Self { commit_type_descript, typ, scope, subject, body, emoji: None }
	}

	/// Load externally provided extension types.
	pub fn load_ext_td(mut self, t: &Vec<String>) -> Self {
		let mut td = t
			.iter()
			.map(|typ: &String| -> CommitTD {
				let arr_td = typ.split(SEPARATOR_SYMBOL).collect::<Vec<&str>>();
				if arr_td.len() < 2 {
					grc_err_println(TYPE_PARSE_FAILED);
					std::process::exit(1);
				};
				CommitTD::from(arr_td[0].trim(), arr_td[1].trim())
			})
			.collect::<Vec<CommitTD>>();
		self.commit_type_descript.append(&mut td);
		self.append_custom_td();
		self
	}

	/// Load externally provided extension emoji.
	/// This overrides the basic emoji settings.
	pub fn load_ext_emoji(mut self, emo: &Vec<String>) -> Self {
		for td in &mut self.commit_type_descript {
			emo.iter().for_each(|type_emo: &String| {
				let arr_emo = type_emo.split(SEPARATOR_SYMBOL).collect::<Vec<&str>>();
				if arr_emo.len() < 2 {
					grc_err_println(OVERWRITE_PARSE_FAILED);
					std::process::exit(1);
				};
				if td.0 == arr_emo[0].trim() {
					td.update_emoji(arr_emo[1]);
					return;
				}
			})
		}

		self
	}

	// got commit message for self.
	pub fn ask(mut self) -> Self {
		self.ask_type();
		self.ask_scope();
		self.ask_subject();
		self.build_body();
		self
	}

	/// generate commit message.
	pub fn build(&self) -> String {
		let header = if self.scope.trim().len() == 0 {
			format!("{}: {}", self.typ, self.subject)
		} else {
			format!("{}({}): {}", self.typ, self.scope, self.subject)
		};

		if self.body.trim().len() == 0 {
			header
		} else {
			format!("{} \n\n{}", header, self.body)
		}
	}

	fn append_custom_td(&mut self) {
		self.commit_type_descript.push(CommitTD::from("<~>", "Define your commit type."))
	}

	/// generate commit long description.
	fn build_body(&mut self) {
		let description = self.ask_description();
		let closes = self.ask_close();

		let mut body = String::new();

		if description.len() > 0 {
			body = description;
		};

		if closes.len() > 0 {
			body = format!("{}\n\nClose #{}", body, closes);
		};

		self.body = body
	}

	/// type of commit message.
	fn ask_type(&mut self) {
		let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
			.items(&self.type_list())
			.default(0)
			.interact()
			.expect("Failed to select commit type.");

		// Custom TYPE.
		if selection == self.commit_type_descript.len() - 1 {
			self.typ = Input::with_theme(&ColorfulTheme::default())
				.with_prompt("What Type ?")
				.validate_with(|input: &String| -> Result<(), &str> {
					if input.len() == 0 || input.trim().len() == 0 {
						Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
					} else {
						Ok(())
					}
				})
				.interact()
				.unwrap()
		} else {
			self.typ = self.commit_type_descript[selection].0.to_owned();
			for td in &self.commit_type_descript {
				if self.typ == td.0 {
					self.emoji = td.2.to_owned();
					break;
				}
			}
		}
	}

	/// scope of commit message.
	fn ask_scope(&mut self) {
		let scope = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("Which scope? (Optional)")
			.allow_empty(true)
			.interact()
			.unwrap();

		self.scope = String::from(scope.trim())
	}

	/// subject of commit message.
	fn ask_subject(&mut self) {
		self.subject = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("Commit Message ?")
			.validate_with(|input: &String| -> Result<(), &str> {
				if input.len() == 0 || input.trim().len() == 0 {
					Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
				} else {
					Ok(())
				}
			})
			.interact()
			.unwrap();

		if !self.emoji.is_some() {
			self.subject = format!("{} {}", self.emoji.as_deref().unwrap(), self.subject)
		}
	}

	/// description of commit message.
	fn ask_description(&self) -> String {
		let has_description = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt("Provide a longer description?")
			.default(false)
			.interact()
			.unwrap_or(false);

		if has_description {
			Editor::new()
				.edit("commit descript.")
				.unwrap_or_else(|_| Some("".to_owned()))
				.unwrap_or_else(|| "".to_owned())
		} else {
			"".to_owned()
		}
	}

	/// close PR or issue of commit message.
	fn ask_close(&self) -> String {
		let closes = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("PR & Issues this commit closes, e.g 123: (Optional)")
			.allow_empty(true)
			.interact()
			.unwrap();

		String::from(remove_pound_prefix(closes.trim()))
	}

	// self.commit_type_descript { Vec<CommitTD> } convert to { Vec<String> }.
	fn type_list(&self) -> Vec<String> {
		let name_max_len = self
			.commit_type_descript
			.iter()
			.reduce(|long_td, td| if td.0.len() > long_td.0.len() { td } else { long_td })
			.unwrap()
			.0
			.len() + 2;

		self.commit_type_descript
			.iter()
			.map(|td: &CommitTD| -> String {
				let mut space = String::new();
				if td.0.len() < name_max_len {
					for _ in 0..(name_max_len - 1 - td.0.len()) {
						space.push_str(SPACE);
					}
				}

				if td.2.is_none() {
					format!("   {}:{}{}", td.0, space, td.1)
				} else {
					format!("{} {}:{}{}", td.2.as_deref().unwrap(), td.0, space, td.1)
				}
			})
			.collect::<Vec<String>>()
	}
}

impl Default for Messager {
	fn default() -> Self {
		Self::new(false)
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_build() {
		let mut message = Messager::new(false);
		message.typ = "type".to_owned();
		message.scope = "scope".to_owned();
		message.subject = "subject.".to_owned();

		let commit_msg = message.build();
		assert_eq!(commit_msg.as_str(), "type(scope): subject.")
	}
}
