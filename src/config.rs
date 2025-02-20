use core::str;
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

use crate::common::{extract_file_extenstion, read_file, AnubisError};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AnubisConfig {
    pub url: String,
    pub language_configs: HashMap<String, LanguageConfig>,
    pub anubis_ignore: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct LanguageConfig {
    pub language: String,
    pub anubis_character: String,
    pub multiline_start: String,
    pub multiline_end: String,
}

impl AnubisConfig {
    pub fn get_language_config(
        &self,
        file: &Path,
    ) -> Result<&LanguageConfig, Box<dyn std::error::Error>> {
        let file_extenstion = extract_file_extenstion(file).ok_or("No File Extenstion Supplied")?;
        self.language_configs
            .get(file_extenstion)
            .ok_or(format!("Config not found for file: {:?}", file).into())
    }

    pub fn generate_ignore_glob(&self) -> Result<GlobSet, Box<dyn std::error::Error>> {
        let mut glob_builder = GlobSetBuilder::new();
        for ignore_pattern in &self.anubis_ignore {
            glob_builder.add(Glob::new(ignore_pattern)?);
        }
        Ok(glob_builder.build()?)
    }

    pub fn deserialize_config(
        config_path: Option<&PathBuf>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = config_path {
            let file_contents = read_file(path)?;
            Ok(serde_json::from_str(&file_contents)?)
        } else {
            Self::search_for_config()
        }
    }

    pub fn search_for_config() -> Result<Self, Box<dyn std::error::Error>> {
        for entry in WalkDir::new("./").max_depth(1) {
            let file = &entry?.into_path();
            if let Some(extenstion) = extract_file_extenstion(file) {
                if extenstion == "anubis" {
                    return Self::deserialize_config(Some(file));
                }
            };
        }
        Err(Box::new(AnubisError::ConfigError(
            "Unable to find anubis config file".to_string(),
        )))
    }
}
