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
        let file_str = Self::read_config_file(GRC_CONFIG_FILE_NAME)?;
        Ok(Self::deserialize(file_str)?)
    }

    fn deserialize(file_str: String) -> Result<Option<Self>, Error> {
        if file_str.len() == 0 || file_str == "" {
            return Ok(None);
        }

        let config = toml::from_str(file_str.as_str())?;
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

    use std::fs;
    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn it_read_config_file() {
        let file_str = GrcConfig::read_config_file("grc.test.toml").unwrap();
        assert_eq!(file_str.as_str(), "type = \"123\"");

        let file_str2 = GrcConfig::read_config_file("nullfile").unwrap();
        assert_eq!(file_str2.len(), 0);
        assert_eq!(file_str2.as_str(), "");
    }
}
