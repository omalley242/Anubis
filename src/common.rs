use std::{collections::HashMap, error::Error, fmt};

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Config
{
    pub language_configs: HashMap<String, LanguageConfig>,
    pub anubis_ignore: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageConfig
{
    pub anubis_character: String,
    pub multiline_start: String,
    pub multiline_end: String,
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            language_configs: HashMap::new(),
            anubis_ignore: vec![]
        }
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        LanguageConfig {
                anubis_character: "".to_string(), 
                multiline_start: "".to_string(),
                multiline_end: "".to_string()
            }
    }
}


#[derive(Debug, Clone)]
pub enum AnubisError{
    ParsingError(String),
}

impl fmt::Display for AnubisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Error occured when running anubis:
                {self}
            "
        )
    }

}

//implement this for each error type
impl std::error::Error for AnubisError{
    fn cause(&self) -> Option<&dyn Error> {
        return Some(self)   
    }

    fn description(&self) -> &str {
        match self {
            Self::ParsingError(desc) => return desc
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return Some(self);
    }
}
