use std::fs;

use serde::Deserialize;

use crate::metadata::GRC_CONFIG_FILE_NAME;

#[derive(Deserialize)]
pub struct GrcConfig {
    #[serde(rename = "type")]
    typ: Option<Vec<String>>,
}

impl GrcConfig {
    // pub fn new() -> Result<Option<Self>, String> {
    // 	fs::read_to_string(GRC_CONFIG_FILE_NAME)
    // }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn grconf_deserializer() {
        let file = match fs::read_to_string(GRC_CONFIG_FILE_NAME) {
            Ok(content) => content,
            Err(e) => panic!(e),
        };

        let config: GrcConfig = toml::from_str(file.as_str()).unwrap();

        assert_ne!(config.typ, None);
        let config_type: Vec<String> = config.typ.unwrap();

        assert_eq!(config_type.len(), 1);
        let version = config_type[0].clone();

        assert_eq!(version.as_str(), "version: version is change.");
    }
}
