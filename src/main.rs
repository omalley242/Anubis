use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Anubis")]
#[command(version = "0.1")]
#[command(about = "The Anubis CLI for tightly intergrated clean documentation", long_about = None)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub database_cache: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub template_directory: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub parsing_directory: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Parse,
    Render,
    Serve,
    All,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = AnubisConfig::deserialize_config(cli.config.as_ref())?;
    let database = AnubisDatabase::new(cli.database_cache.as_ref())?;
    let parser = AnubisParser::new(cli.parsing_directory.as_ref())?;
    let renderer = AnubisRenderer::new(cli.template_directory.as_ref())?;
    let server = AnubisServer::new();

    let anubis = Anubis {
        config,
        database,
        parser,
        renderer,
        server,
    };

    match cli.command {
        Some(Commands::Parse) => anubis.parse(),
        Some(Commands::Render) => anubis.render(),
        Some(Commands::Serve) => anubis.serve().await,
        Some(Commands::All) | None => {
            anubis.parse()?;
            anubis.render()?;
            anubis.serve().await
        }
    }
}
