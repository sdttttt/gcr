use std::fs;
use std::io::{Error, ErrorKind};

use serde::Deserialize;

use crate::metadata::GRC_CONFIG_FILE_NAME;

#[derive(Deserialize)]
pub struct GrcConfig {
    #[serde(rename = "type")]
    typ: Option<Vec<String>>,
}

impl GrcConfig {
    pub fn from_agreement() -> Result<Option<Self>, Error> {
        let file_str = Self::read_config_file(GRC_CONFIG_FILE_NAME)?;
        Ok(Self::deserialize(file_str)?)
    }

    pub fn from(filename: &str) -> Result<Option<Self>, Error> {
        let file_str = Self::read_config_file(filename)?;
        Ok(Self::deserialize(file_str)?)
    }

    pub fn types(&self) -> &Option<Vec<String>> {
        &self.typ
    }

    fn deserialize(file_str: String) -> Result<Option<Self>, Error> {
        if file_str.len() == 0 || file_str == "" {
            return Ok(None);
        }

        let config = toml::from_str::<GrcConfig>(file_str.as_str())?;
        Ok(Some(config))
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
        let file_str = GrcConfig::read_config_file(GRC_TEST_CONFIG_FILE_NAME).unwrap();
        assert_eq!(file_str.as_str(), GRC_TEST_TOML_CONTENT);

        let file_str2 = GrcConfig::read_config_file("nullfile").unwrap();
        assert_eq!(file_str2.len(), 0);
        assert_eq!(file_str2.as_str(), "");

        let config = GrcConfig::read_config_file(GRC_CONFIG_FILE_NAME).unwrap();
        assert_eq!(config.as_str(), GRC_TOML_CONTENT);
    }

    #[test]
    fn it_deserialize() {
        let file_str = String::from(GRC_TOML_CONTENT);
        let result = GrcConfig::deserialize(file_str).unwrap().unwrap();

        let types = result.types().as_ref().unwrap();
        assert_eq!(types[0], GRC_TOML_TYPE);
    }

    // #[test]
    // fn it_from_agreement() {
    //     let config = GrcConfig::from_agreement().unwrap().unwrap();
    //     let types = config.types().as_ref().unwrap();
    // }
}
