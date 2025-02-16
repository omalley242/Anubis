use crate::renderer::render_files;
use crate::server::run_server;
use crate::{common::deserialize_config, parser::parse};

use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(name = "Anubis")]
#[command(version = "0.1")]
#[command(about = "The Anubis CLI for tightly intergrated clean documentation", long_about = None)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub debug: bool,

    /// Which command to run
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Parses the supplied repository and builds the block database
    Parse,
    /// Renders the blocks within the database
    Render,
    /// Runs the built in webserver
    Run,
    /// Runs both the entire pipeline
    All,
}

pub fn process_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = deserialize_config(cli.config.as_ref())?;
    match cli.command {
        Some(Commands::Parse) => parse(&config),
        Some(Commands::Render) => render_files(&config),
        Some(Commands::Run) => run_server(&config),
        Some(Commands::All) | None => {
            parse(&config)?;
            render_files(&config)?;
            run_server(&config)
        }
    }
}
