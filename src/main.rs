use core::str;
use std::{collections::HashSet, fs::File, io::{BufReader, Read}, path::PathBuf};
use anubis::{common::{Config, LanguageConfig}, parser::{parse_file_contents, Block}};
use clap::{Parser, Subcommand};
use globset::{Glob, GlobSet, GlobSetBuilder};
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

    let _run_response = match cli.command {
        Some(Commands::Parse) => {parse_files(config_path.as_ref())}
        Some(Commands::Run) => {run_server()}
        Some(Commands::All) | None => {
            let parse_error = parse_files(config_path.as_ref());

            if parse_error.is_err() {
                println!("{:?}", parse_error);
                parse_error
            }else{
                run_server()
            }

        }
    };

    //Error Handling managed here

}

fn parse_files(config_path: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {

    let config = deserialize_config(config_path)?;          //obtain config

    let mut file_worklist = collect_all_files();  //collect all possible files

    let ignore_glob = generate_ignore_glob(&config)?;      //create ignore glob

    remove_ignored_files(&mut file_worklist, ignore_glob);    //apply ignore glob

    for file in file_worklist {
        
        let lang_config = match find_language_config(&file, &config) {
            Some(config) => config,
            None => continue
        };

        let blocks = parse_file(&file, lang_config)?;
        
        println!("{:?}", blocks);
        
        //add data to database


    }

    Ok(())
}

fn remove_ignored_files<'a>(file_list: &mut HashSet<PathBuf>, ignore_glob: GlobSet){
    file_list.retain(|file| !ignore_glob.is_match(file));
} 

fn collect_all_files() -> HashSet<PathBuf> {
    WalkDir::new("./")
    .into_iter()
    .filter_map(|file| file.ok()) //Remove non-ok values
    .filter(|file| file.path().is_file())
    .map(|file| file
        .path()
        .to_path_buf())
        .collect::<HashSet<PathBuf>>()
}

fn generate_ignore_glob<'a>(config: &'a Config) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut glob_builder = GlobSetBuilder::new();

    for ignore_pattern in &config.anubis_ignore {
        glob_builder.add(Glob::new(&ignore_pattern)?);
    }

    Ok(glob_builder.build()?)
}


fn extract_file_extenstion<'a>(file: &'a PathBuf) -> Option<&'a str>{
    let file_os_string = file.file_name()?;
    let file_name_string = file_os_string.to_str()?;    
    return file_name_string.split(".").last()
}

fn find_language_config<'a>(file: &PathBuf, config: &'a Config) -> Option<&'a LanguageConfig> {
    let file_extenstion = extract_file_extenstion(file)?;
    return config.language_configs.get(file_extenstion)
}

fn parse_file<'a>(file_path: &'a PathBuf,  language_config: &'a LanguageConfig) -> Result<Option<Vec<Block>>, Box<dyn std::error::Error>>{
    let file_to_parse = File::open(file_path)?;
    let mut file_reader = BufReader::new(file_to_parse);

    let mut file_contents = String::new();
    let _ = file_reader.read_to_string(&mut file_contents)?;

    return Ok(parse_file_contents(&file_contents, language_config));
}

fn deserialize_config<'a>(config_path: Option<&PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {

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

            let file = &entry?.into_path();

            let extenstion = match extract_file_extenstion(file){
                Some(extenstion) => extenstion,
                None => continue,
            };

            //if the file extenstion is .anubis deserialize the config
            if extenstion == "anubis" {
                return deserialize_config(Some(file));
            }
        }

        // if not anubis file is found use the default config
        return Ok(Config::default());
    }
}

fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
