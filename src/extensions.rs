use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

use serde::Deserialize;

use crate::metadata::{GLOBAL_CONFIG_PATH, GRC_CONFIG_FILE_NAME};

/// Extensions is GRC future config.
#[derive(Deserialize)]
pub struct Extensions {
	#[serde(rename = "type")]
	typ: Option<Vec<String>>,

	emoji: Option<bool>,

	overwrite_emoji: Option<Vec<String>>,
	plug: Option<Vec<String>>,

	pre: Option<Vec<String>>,
	after: Option<Vec<String>>,
}

impl Extensions {
	/// Read Extension from the configuration file in the convention name.
	pub fn from_agreement() -> Result<Self, Error> {
		let file_str = Self::read_config_file(GRC_CONFIG_FILE_NAME)?;
		if file_str.len() == 0 {
			Self::from_global()
		} else {
			Ok(Self::deserialize(file_str)?)
		}
	}

	#[cfg(not(target_os = "windows"))]
	fn from_global() -> Result<Self, Error> {
		let home_dir = env::var("HOME").unwrap_or(String::new());
		let file_path = format!("{}/{}", home_dir.as_str(), GLOBAL_CONFIG_PATH);
		Self::from(file_path.as_str())
	}

	#[cfg(target_os = "windows")]
	fn from_global() -> Result<Self, Error> {
		let home_dir = env::var("USERPROFILE").unwrap_or(String::new());
		let file_path = format!("{}\\{}", home_dir.as_str(), GLOBAL_CONFIG_PATH);
		Self::from(file_path.as_str())
	}

	/// Read Extension from the configuration file in the Specified name.
	pub fn from(filename: &str) -> Result<Self, Error> {
		let file_str = Self::read_config_file(filename)?;
		Ok(Self::deserialize(file_str)?)
	}

	/// got All Types in configuration file.
	pub fn types(&self) -> Option<&Vec<String>> {
		self.typ.as_ref()
	}

	pub fn emoji(&self) -> bool {
		self.emoji.unwrap_or_else(|| false)
	}

	pub fn overwrite_emoji(&self) -> Option<&Vec<String>> {
		self.overwrite_emoji.as_ref()
	}

	pub fn plug(&self) -> Option<&Vec<String>> {
		self.plug.as_ref()
	}

	pub fn pre_command(&self) -> Option<&Vec<String>> {
		self.pre.as_ref()
	}

	pub fn after_command(&self) -> Option<&Vec<String>> {
		self.after.as_ref()
	}

	/// deserialize toml configuration file to struct.
	fn deserialize(file_str: String) -> Result<Self, Error> {
		if file_str.len() == 0 || file_str == "" {
			return Ok(Self {
				typ: None,
				emoji: None,
				overwrite_emoji: None,
				plug: None,
				pre: None,
				after: None,
			});
		}

		let config = toml::from_str::<Extensions>(file_str.as_str())?;
		Ok(config)
	}

	/// read config file convert std::string::String
	fn read_config_file(filename: &str) -> Result<String, Error> {
		match fs::read_to_string(filename) {
			Ok(content) => Ok(content),
			Err(e) => {
				if e.kind() == ErrorKind::NotFound {
					Ok(String::new())
				} else {
					Err(e)
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	const GRC_TEST_CONFIG_FILE_NAME: &str = "grc.test.toml";

	const GRC_TOML_TYPE: &str = "version: version is change.";
	const GRC_TOML_EMOJI: Option<bool> = Some(true);

	const GRC_TEST_TOML_CONTENT: &str = r#"type = [ "123" ]"#;
	const GRC_TEST_TOML_TYPE: &str = "123";

	#[test]
	fn it_read_config_file() {
		let file_str = Extensions::read_config_file(GRC_TEST_CONFIG_FILE_NAME).unwrap();
		assert_eq!(file_str.as_str(), GRC_TEST_TOML_CONTENT);

		let file_str2 = Extensions::read_config_file("null_file").unwrap();
		assert_eq!(file_str2.len(), 0);
		assert_eq!(file_str2.as_str(), "");

		let config = Extensions::read_config_file(GRC_CONFIG_FILE_NAME).unwrap();
		assert!(config.len() > 0);
	}

	#[test]
	fn it_deserialize() {
		let config = Extensions::read_config_file(GRC_CONFIG_FILE_NAME).unwrap();
		let result = Extensions::deserialize(config).unwrap();

		let types = result.typ.unwrap();
		let emoji = result.emoji;
		assert_eq!(types[0], GRC_TOML_TYPE);
		assert_eq!(emoji, GRC_TOML_EMOJI);
	}

	#[test]
	fn it_from_agreement() {
		let config = Extensions::from_agreement().unwrap();
		let types = config.typ.unwrap();

		assert_eq!(types[0], GRC_TOML_TYPE);
	}

	#[test]
	fn it_from() {
		let config = Extensions::from(GRC_TEST_CONFIG_FILE_NAME).unwrap();
		let types = config.typ.unwrap();

		assert_eq!(types[0], GRC_TEST_TOML_TYPE);
	}
}
