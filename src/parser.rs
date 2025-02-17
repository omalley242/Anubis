use crate::common::{find_language_config, remove_ignored_files, Block, Config, LanguageConfig};
use crate::db::{block_db, insert_block};
use crate::parser_core::top;
use core::str;
use globset::{Glob, GlobSet, GlobSetBuilder};
use nom::Parser;
use rusqlite::Connection;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use walkdir::WalkDir;

pub fn parse(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let db_con = block_db()?; //create connection and table
    let mut file_list = collect_all_files(); //collect all possible files
    let ignore_glob = generate_ignore_glob(config)?; //create ignore glob
    remove_ignored_files(&mut file_list, ignore_glob); //apply ignore glob
    parse_files(file_list, config, &db_con)?; //parse the files and add to db
    Ok(())
}

fn parse_files(
    file_list: HashSet<PathBuf>,
    config: &Config,
    db: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in file_list {
        let lang_config = find_language_config(&file, config)?;
        if let Some(blocks) = parse_file(&file, lang_config)? {
            blocks
                .iter()
                .try_for_each(|block| insert_block(db, block, &file))?;
        };
    }
    Ok(())
}

fn parse_file(
    file_path: &PathBuf,
    language_config: &LanguageConfig,
) -> Result<Option<Vec<Block>>, Box<dyn std::error::Error>> {
    let file_to_parse = File::open(file_path)?;
    let mut file_reader = BufReader::new(file_to_parse);
    let mut file_contents = String::new();
    let _ = file_reader.read_to_string(&mut file_contents)?;
    Ok(parse_file_contents(&file_contents, language_config))
}

pub fn parse_file_contents(
    file_contents: &str,
    language_config: &LanguageConfig,
) -> Option<Vec<Block>> {
    if let Ok(result) = top(language_config).parse(file_contents) {
        Some(result.1)
    } else {
        None
    }
}

fn collect_all_files() -> HashSet<PathBuf> {
    WalkDir::new("./")
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|file| file.path().is_file())
        .map(|file| file.path().to_path_buf())
        .collect::<HashSet<PathBuf>>()
}

fn generate_ignore_glob(config: &Config) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut glob_builder = GlobSetBuilder::new();
    for ignore_pattern in &config.anubis_ignore {
        glob_builder.add(Glob::new(ignore_pattern)?);
    }
    Ok(glob_builder.build()?)
}
