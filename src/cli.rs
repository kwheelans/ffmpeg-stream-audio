use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(value_parser)]
    pub configuration: PathBuf,
}