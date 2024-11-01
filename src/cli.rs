use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(value_parser)]
    pub configuration: PathBuf,
}
