use clap::Parser;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(value_parser)]
    pub configuration: PathBuf,

    #[clap(short, long, value_parser)]
    pub log_level: Option<LevelFilter>,
}
