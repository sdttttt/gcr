use std::fs;
use std::io::{Error, ErrorKind};

use serde::Deserialize;

use crate::metadata::GRC_CONFIG_FILE_NAME;

#[derive(Deserialize)]
pub struct Extensions {
    #[serde(rename = "type")]
    typ: Vec<String>,
}

impl Extensions {
    pub fn from_agreement() -> Result<Self, Error> {
        let file_str = Self::read_config_file(GRC_CONFIG_FILE_NAME)?;
        Ok(Self::deserialize(file_str)?)
    }

    pub fn from(filename: &str) -> Result<Self, Error> {
        let file_str = Self::read_config_file(filename)?;
        Ok(Self::deserialize(file_str)?)
    }

    pub fn types(&self) -> &Vec<String> {
        &self.typ
    }

    fn deserialize(file_str: String) -> Result<Self, Error> {
        if file_str.len() == 0 || file_str == "" {
            return Ok(Self { typ: vec![] });
        }

        let config = toml::from_str::<Extensions>(file_str.as_str())?;
        Ok(config)
    }

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

    const GRC_TOML_CONTENT: &str = r#"type = [ "version: version is change." ]"#;
    const GRC_TOML_TYPE: &str = "version: version is change.";

    const GRC_TEST_TOML_CONTENT: &str = r#"type = [ "123" ]"#;
    const GRC_TEST_TOML_TYPE: &str = "123";

    #[test]
    fn it_read_config_file() {
        let file_str = Extensions::read_config_file(GRC_TEST_CONFIG_FILE_NAME).unwrap();
        assert_eq!(file_str.as_str(), GRC_TEST_TOML_CONTENT);

        let file_str2 = Extensions::read_config_file("nullfile").unwrap();
        assert_eq!(file_str2.len(), 0);
        assert_eq!(file_str2.as_str(), "");

        let config = Extensions::read_config_file(GRC_CONFIG_FILE_NAME).unwrap();
        assert_eq!(config.as_str(), GRC_TOML_CONTENT);
    }

    #[test]
    fn it_deserialize() {
        let file_str = String::from(GRC_TOML_CONTENT);
        let result = Extensions::deserialize(file_str).unwrap();

        let types = result.typ;
        assert_eq!(types[0], GRC_TOML_TYPE);
    }

    #[test]
    fn it_from_agreement() {
        let config = Extensions::from_agreement().unwrap();
        let types = config.typ;

        assert_eq!(types[0], GRC_TOML_TYPE);
    }

    #[test]
    fn it_from() {
        let config = Extensions::from(GRC_TEST_CONFIG_FILE_NAME).unwrap();
        let types = config.typ;

        assert_eq!(types[0], GRC_TEST_TOML_TYPE);
    }
}
