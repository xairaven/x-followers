use crate::services::Service;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Alex Kovalov", version = "0.0.1")]
pub struct Cli {
    #[arg(short = 'n', long, help = "User nickname.")]
    pub nickname: String,

    #[arg(short = 's', long, value_enum, help = "Service (Github, ..).")]
    pub service: Service,

    #[arg(
        short = 'd',
        action,
        long,
        help = "Exclude description from the first line."
    )]
    pub exclude_description: bool,

    #[arg(
        short = 'o',
        long,
        help = "Path to the output folder where results will be saved."
    )]
    pub output_path: Option<PathBuf>,

    #[arg(short = 'f', long, help = "Output file name.")]
    pub file_name: Option<String>,
}
