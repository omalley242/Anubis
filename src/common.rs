use core::str;
use globset::GlobSet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockInfo {
    pub name: String,          //Page Name
    pub template_name: String, //Template to use when rendering
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub info: BlockInfo,
    pub content: Vec<BlockContent>, //the markdown content within the block
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockContent {
    Markdown(String), // Markdown content
    Code(String),     // Code string
    Link(String),     // Link to anotherblock
    Embed(String),    // Block to be rendered
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub language_configs: HashMap<String, LanguageConfig>,
    pub anubis_ignore: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageConfig {
    pub language: String,
    pub anubis_character: String,
    pub multiline_start: String,
    pub multiline_end: String,
}

/*@[Defaults|DefaultsPage]
    {{ConfigDefault}}
    {{LanguageConfigDefault}}
@*/

/*@[ConfigDefault|Default]
    # Config Default
*/
impl Default for Config {
    fn default() -> Self {
        Config {
            url: String::new(),
            language_configs: HashMap::new(),
            anubis_ignore: vec![],
        }
    }
}
/*@*/

/*@[LanguageConfigDefault|Default]
    # Language Config Default
*/
impl Default for LanguageConfig {
    fn default() -> Self {
        LanguageConfig {
            language: String::new(),
            anubis_character: String::new(),
            multiline_start: String::new(),
            multiline_end: String::new(),
        }
    }
}
/*@*/

#[derive(Debug, Clone)]
pub enum AnubisError {
    ParsingError(String),
    ConfigError(String),
    RecursiveTemplateError(String),
    PageNotFoundError(String),
    BlockNotFoundError(String),
}

impl fmt::Display for AnubisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error occured when running anubis:
                {self}
            "
        )
    }
}

//implement this for each error type
impl std::error::Error for AnubisError {
    fn cause(&self) -> Option<&dyn Error> {
        return Some(self);
    }

    fn description(&self) -> &str {
        match self {
            AnubisError::ParsingError(desc) => desc,
            AnubisError::ConfigError(desc) => desc,
            AnubisError::RecursiveTemplateError(desc) => desc,
            AnubisError::PageNotFoundError(desc) => desc,
            AnubisError::BlockNotFoundError(desc) => desc,
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return Some(self);
    }
}

pub fn deserialize_config<'a>(
    config_path: Option<&PathBuf>,
) -> Result<Config, Box<dyn std::error::Error>> {
    //Deserialize config file
    if config_path.is_some() {
        let config_file = File::open(config_path.unwrap())?;
        let mut config_reader = BufReader::new(config_file);
        let contents: &mut String = &mut String::new();
        let _ = config_reader.read_to_string(contents)?;

        return Ok(serde_json::from_str::<Config>(&contents)?);
    } else {
        //search the current directory
        for entry in WalkDir::new("./").max_depth(1) {
            let file = &entry?.into_path();

            let extenstion = match extract_file_extenstion(file) {
                Some(extenstion) => extenstion,
                None => continue,
            };

            //if the file extenstion is .anubis deserialize the config
            if extenstion == "anubis" {
                return deserialize_config(Some(file));
            }
        }

        // if not anubis file is found use the default config
        return Err(Box::new(AnubisError::ConfigError(
            "Unable to find anubis config file".to_string(),
        )));
    }
}

pub fn extract_file_extenstion<'a>(file: &'a PathBuf) -> Option<&'a str> {
    let file_os_string = file.file_name()?;
    let file_name_string = file_os_string.to_str()?;
    return file_name_string.split(".").last();
}

pub fn find_language_config<'a>(
    file: &PathBuf,
    config: &'a Config,
) -> Result<&'a LanguageConfig, Box<dyn std::error::Error>> {
    let file_extenstion = extract_file_extenstion(file).ok_or("No File Extenstion Supplied")?;
    config
        .language_configs
        .get(file_extenstion)
        .ok_or(format!("Config not found for file: {:?}", file).into())
}

pub fn remove_ignored_files<'a>(file_list: &mut HashSet<PathBuf>, ignore_glob: GlobSet) {
    file_list.retain(|file| !ignore_glob.is_match(file));
}
