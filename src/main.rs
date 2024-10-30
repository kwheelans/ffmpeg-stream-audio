mod config;
mod error;
mod cli;

use crate::cli::CliArgs;
use crate::config::StreamConfig;
use crate::error::Error;
use clap::Parser;
use std::process::{Command, ExitCode, Stdio};

fn main() -> ExitCode {
    match run() {
        Err(error) => {
            println!("{}", error);
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }

}

fn run () -> Result<(), Error>{
    let cli = CliArgs::parse();
    let config = StreamConfig::try_from(std::fs::read_to_string(cli.configuration)?.as_str())?;
    println!("{:?}", config);
    
    let mut ffmpeg = Command::new("ffmpeg")
        .stdout(Stdio::piped())
        .args(config.to_vec())
        .spawn()?;


    println!("{:?}", ffmpeg);
    //let stdout = ffmpeg.stdout.take().unwrap();

    _ = ffmpeg.wait();
    println!("process exited");
    Ok(())
}
