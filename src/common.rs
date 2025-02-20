use core::str;
use globset::GlobSet;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Read;
use std::{collections::HashSet, fs::File, io::BufReader, path::Path, path::PathBuf};
use tera::Tera;
use walkdir::WalkDir;

use crate::config::AnubisConfig;
use crate::db::AnubisDatabase;

pub struct Anubis {
    pub database: AnubisDatabase,
    pub config: AnubisConfig,
    pub tera: Tera,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockInfo {
    pub name: String,
    pub template_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub info: BlockInfo,
    pub content: Vec<BlockContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockContent {
    Markdown(String),
    Code(String),
    Link(String),
    Embed(String),
}

/*@[Anubis Error|join]
{{AnubisErrorEnum}}
{{AnubisErrorImpls}}
@*/

/*@[AnubisErrorEnum|Enum]
# Anubis Error
Implements a typed enum containing an error message
*/
#[derive(Debug, Clone)]
pub enum AnubisError {
    ParsingError(String),
    ConfigError(String),
    RecursiveTemplateError(String),
    PageNotFoundError(String),
    BlockNotFoundError(String),
    ConnectionsNotFound(String),
    ContextError(String),
}
/*@*/

/*@[AnubisErrorImpls|Impl]
# Anubis Error implementations
Traits that have been implemented for the Anubis Error Type
*/
impl fmt::Display for AnubisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occured when running anubis")
    }
}

impl std::error::Error for AnubisError {
    fn description(&self) -> &str {
        match self {
            AnubisError::ParsingError(desc) => desc,
            AnubisError::ConfigError(desc) => desc,
            AnubisError::RecursiveTemplateError(desc) => desc,
            AnubisError::PageNotFoundError(desc) => desc,
            AnubisError::BlockNotFoundError(desc) => desc,
            AnubisError::ConnectionsNotFound(desc) => desc,
            AnubisError::ContextError(desc) => desc,
        }
    }
}
/*@*/

pub fn extract_file_extenstion(file: &Path) -> Option<&str> {
    let file_os_string = file.file_name()?;
    let file_name_string = file_os_string.to_str()?;
    file_name_string.split(".").last()
}

pub fn remove_ignored_files(file_list: &mut HashSet<PathBuf>, ignore_glob: GlobSet) {
    file_list.retain(|file| !ignore_glob.is_match(file));
}

pub fn read_file(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let file_to_parse = File::open(file_path)?;
    let mut file_reader = BufReader::new(file_to_parse);
    let mut file_contents = String::new();
    let _ = file_reader.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

pub fn collect_all_files() -> HashSet<PathBuf> {
    WalkDir::new("./")
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|file| file.path().is_file())
        .map(|file| file.path().to_path_buf())
        .collect::<HashSet<PathBuf>>()
}
