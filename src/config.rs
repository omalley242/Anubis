use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

use crate::error::AnubisError;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisConfigInterface {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>>;
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct LanguageConfig {
    pub language: String,
    pub syntax: String,
    pub line: String,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct AnubisConfig {
    #[serde(default)]
    pub url: Option<String>,

    #[serde(default)]
    pub template_directory: Option<String>,

    #[serde(default)]
    pub anubis_ignore: Option<Vec<PathBuf>>,

    pub language_configs: HashMap<String, LanguageConfig>,
}

impl AnubisConfigInterface for AnubisConfig {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>> {
        if let Some(path) = path {
            let contents = fs::read_to_string(path)?;
            Ok(serde_json::from_str::<AnubisConfig>(&contents)?.into())
        } else {
            search_config()
        }
    }
}

fn search_config() -> Result<Box<AnubisConfig>> {
    for entry in WalkDir::new("./").max_depth(1) {
        let file = entry?.into_path();
        if let Some(extenstion) = file.extension() {
            if extenstion == "anubis" {
                return AnubisConfig::new(Some(&file));
            }
        };
    }
    Err(AnubisError::ConfigError.into())
}
