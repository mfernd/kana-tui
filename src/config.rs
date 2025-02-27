use crate::models;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    ConfigNotFound,
    InvalidConfig(toml::de::Error),
    SerializationError(toml::ser::Error),
    FailedToSave(std::io::Error),
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    writing_system: WritingSystem,
}

impl Config {
    pub fn parse_from_path(config_path: &PathBuf) -> Result<Self, ConfigError> {
        let file = std::fs::read_to_string(config_path).map_err(|_| ConfigError::ConfigNotFound)?;
        toml::from_str(&file).map_err(ConfigError::InvalidConfig)
    }

    pub fn save(&self, config_path: &PathBuf) -> Result<(), ConfigError> {
        let toml = toml::to_string_pretty(self).map_err(ConfigError::SerializationError)?;
        std::fs::write(config_path, toml).map_err(ConfigError::FailedToSave)
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub enum WritingSystem {
    #[default]
    Hiragana,
    Katakana,
}

impl From<models::kana::KanaRepresentation> for WritingSystem {
    fn from(value: models::kana::KanaRepresentation) -> Self {
        match value {
            models::kana::KanaRepresentation::Hiragana => Self::Hiragana,
            models::kana::KanaRepresentation::Katakana => Self::Katakana,
        }
    }
}
