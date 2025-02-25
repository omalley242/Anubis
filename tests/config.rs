use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anubis::config::{AnubisConfig, AnubisConfigInterface, LanguageConfig};

#[test]
fn test_new_config_with_path() {
    let path = PathBuf::from_str("./tests/test_config.anubis").unwrap();
    let config: AnubisConfig = *AnubisConfigInterface::new(Some(&path)).unwrap();
    let mut language_configs = HashMap::new();
    language_configs.insert(
        "rs".to_string(),
        LanguageConfig {
            language: "rust".to_string(),
            syntax: "@".to_string(),
            line: "//".to_string(),
            start: "/*".to_string(),
            end: "*/".to_string(),
        },
    );
    let correct_config = AnubisConfig {
        url: Some("http://127.0.0.1:3000/".to_string()),
        template_directory: None,
        anubis_ignore: Some(vec![PathBuf::from_str("*.anubis").unwrap()]),
        language_configs,
    };

    assert_eq!(config, correct_config);
}
