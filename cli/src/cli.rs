use crate::services::Service;
use clap::ArgAction;
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
        action=ArgAction::SetFalse,
        long,
        help = "Include description as the first line."
    )]
    pub include_description: bool,

    #[arg(
        short = 'o',
        long,
        help = "Path to the output file where results will be saved."
    )]
    pub output_path: Option<PathBuf>,

    #[arg(short = 'f', long, help = "Filename.")]
    pub file_name: Option<String>,
}
