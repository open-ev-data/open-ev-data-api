use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "ev-etl")]
#[command(author, version, about = "OpenEV Data ETL Pipeline")]
#[command(long_about = "Transform layered JSON vehicle data into multiple output formats")]
pub struct Cli {
    #[arg(short, long, value_name = "DIR")]
    #[arg(help = "Input directory containing vehicle JSON files")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "DIR", default_value = "./output")]
    #[arg(help = "Output directory for generated artifacts")]
    pub output: PathBuf,

    #[arg(short, long, value_delimiter = ',', default_value = "json,sqlite")]
    #[arg(help = "Output formats: json, sqlite, postgresql, csv, xml")]
    pub formats: Vec<String>,

    #[arg(long)]
    #[arg(help = "Validate without generating output")]
    pub validate_only: bool,

    #[arg(short, long)]
    #[arg(help = "Enable verbose output")]
    pub verbose: bool,
}
