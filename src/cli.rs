use crate::common::deserialize_config;
use crate::common::Anubis;
use crate::db::AnubisDatabase;
use crate::parser::AnubisParser;
use crate::renderer::AnubisRenderer;
use crate::server::AnubisServer;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tera::Tera;
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

pub async fn process_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = deserialize_config(cli.config.as_ref())?;
    let database = AnubisDatabase::new(None)?;
    let tera = Tera::new("./default_templates/**/*.html")?;

    let mut anubis = Anubis {
        config,
        database,
        tera,
    };

    match cli.command {
        Some(Commands::Parse) => anubis.parse(),
        Some(Commands::Render) => anubis.render(),
        Some(Commands::Run) => anubis.serve().await,
        Some(Commands::All) | None => {
            anubis.parse()?;
            anubis.render()?;
            anubis.serve().await
        }
    }
}
