use crate::models;
use std::{path::PathBuf, sync::LazyLock};

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let config_folder = dirs::config_dir()
        .expect("Config folder for your OS not found")
        .join(env!("CARGO_PKG_NAME"));

    if !config_folder.exists() {
        std::fs::create_dir_all(&config_folder).expect("Could not create config folder");
    }

    config_folder.join("config.toml")
});

#[derive(Debug)]
pub enum ConfigError {
    ConfigNotFound,
    InvalidConfig(toml::de::Error),
    SerializationError(toml::ser::Error),
    FailedToSave(std::io::Error),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub writing_system: WritingSystem,
}

impl Default for Config {
    fn default() -> Self {
        Config::parse_from_path(&CONFIG_PATH).unwrap_or_else(|_| {
            let default_config = Config {
                writing_system: WritingSystem::default(),
            };
            default_config.save().expect("Could not save config");
            default_config
        })
    }
}

impl Config {
    fn parse_from_path(config_path: &PathBuf) -> Result<Self, ConfigError> {
        let file = std::fs::read_to_string(config_path).map_err(|_| ConfigError::ConfigNotFound)?;
        toml::from_str(&file).map_err(ConfigError::InvalidConfig)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let toml = toml::to_string_pretty(&self).map_err(ConfigError::SerializationError)?;
        std::fs::write(&*CONFIG_PATH, toml).map_err(ConfigError::FailedToSave)
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
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

impl From<WritingSystem> for models::kana::KanaRepresentation {
    fn from(value: WritingSystem) -> Self {
        match value {
            WritingSystem::Hiragana => Self::Hiragana,
            WritingSystem::Katakana => Self::Katakana,
        }
    }
}
