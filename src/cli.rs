use clap::Parser;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(value_parser)]
    pub configuration: PathBuf,
    
    #[clap(short, long, value_parser)]
    pub log_level: Option<LevelFilter>,

    /// Download Pico CSS archive and exit
    #[clap(long, conflicts_with = "configuration")]
    pub download_pico_css: bool,
}
