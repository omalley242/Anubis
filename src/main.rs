use core::str;
use std::{collections::{HashMap, HashSet}, error::Error, fmt::{self}, fs::File, io::{BufReader, Read}, path::PathBuf, str::FromStr, vec};
use anubis::{common::{AnubisError, Config, LanguageConfig}, parser::{parse_file_contents, Block}};
use clap::{Parser, Subcommand};
use globset::{Glob, GlobBuilder, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "Anubis")]
#[command(version = "0.1")]
#[command(about = "The Anubis CLI for tightly intergrated clean documentation", long_about = None)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    /// Which command to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses the supplied repository
    Parse,
    /// Runs the built in webserver
    Run,
    /// Runs both the parser and webserver
    All,
}


fn main() {
    
    let cli = Cli::parse();

    let config_path = cli.config;

    let run_response = match cli.command {
        Some(Commands::Parse) => {parse_files(config_path)}
        Some(Commands::Run) => {run_server()}
        Some(Commands::All) | None => {
            let parse_error = parse_files(config_path);

            if parse_error.is_err() {
                parse_error
            }else{
                run_server()
            }

        }
    };

    //Error Handling managed here

}

fn parse_files(config_path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {

    let config = deserialize_config(config_path)?;

    //Generate Set of all files
    let file_worklist = WalkDir::new("./")
    .into_iter()
    .filter_map(|file| file.ok()) //Remove non-ok values
    .filter(|file| file.path().is_file())
    .map(|file| file
        .path()
        .to_path_buf())
        .collect::<HashSet<PathBuf>>();

    let mut glob_builder = GlobSetBuilder::new();

    //Remove all anubisignored files
    for ignore_pattern in config.anubis_ignore {
        glob_builder.add(Glob::new(&ignore_pattern)?);
    }

    let glob_set = glob_builder.build()?;

    let file_worklist = file_worklist.iter()
    .filter(
        |file| !glob_set.is_match(file))
        .collect::<HashSet<&PathBuf>>();

    for file in file_worklist {
        //parse file
        let file_os_string = match file.file_name() {
            Some(file_os_string) => file_os_string,
            None => continue, 
        };

        let file_name_string = match file_os_string.to_str() {
            Some(file_name_str) => file_name_str.to_string(),
            None => continue,
        };
        
        let file_extenstion = match file_name_string.split(".").last() {
            Some(file_extenstion) => file_extenstion,
            None => continue,
        };

        let lang_config = match config.language_configs.get(file_extenstion) {
            Some(lang_config) => lang_config,
            None => {
                return Err(Box::new(AnubisError::ParsingError(format!("Unable to find config for file extenstion: {file_extenstion}"))));
            }
        };


        let parse_response = parse_file(file, lang_config);
        //add data to database

    }

    Ok(())
}

fn parse_file<'a>(file_path: &'a PathBuf,  language_config: &'a LanguageConfig) -> Result<Vec<Block>, Box<dyn std::error::Error>>{
    //call parser
    let file_to_parse = File::open(file_path)?;
    let mut file_reader = BufReader::new(file_to_parse);

    let mut file_contents = String::new();
    let _ = file_reader.read_to_string(&mut file_contents)?;

    let parse_result = parse_file_contents(&file_contents, language_config);

    println!("{:?}", parse_result);
    println!("{:?}", file_path);

    Ok(vec![])
}

fn deserialize_config<'a>(config_path: Option<PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {

    //Deserialize config file
    if config_path.is_some() {
        let config_file = File::open(config_path.unwrap())?;
        let mut config_reader = BufReader::new(config_file);
        let contents:&mut String  = &mut String::new();
        let _ = config_reader.read_to_string( contents)?;

        return Ok(serde_json::from_str::<Config>(&contents)?);
        
    } else {

        //search the current directory
        for entry in WalkDir::new("./").max_depth(1) {
            let file_name: String = entry?.file_name().to_str().unwrap().to_string();
            let file_parts = file_name.split(".");
            //if the file extenstion is .anubis deserialize the config
            if file_parts.last().unwrap() == "anubis" {
                return deserialize_config(Some(PathBuf::from_str(&file_name)?));
            }
        }

        // if not anubis file is found use the default config
        return Ok(Config::default());
    }
}

fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
