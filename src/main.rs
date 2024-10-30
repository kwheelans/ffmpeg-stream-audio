mod config;
mod error;
mod cli;

use std::process::{ExitCode, Stdio};
use crate::cli::CliArgs;
use crate::config::StreamConfig;
use crate::error::Error;
use clap::Parser;
use tokio::process::Command;

fn main() -> ExitCode {
    match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => {
            match rt.block_on(run()) {
                Err(error) => {
                    println!("{}", error);
                    ExitCode::FAILURE
                }
                Ok(_) => ExitCode::SUCCESS,
            }
        }
        Err(error) => {
            println!("{}", error);
            ExitCode::FAILURE
        }
    }
}

async fn run () -> Result<(), Error>{
    let cli = CliArgs::parse();
    let config = StreamConfig::try_from(std::fs::read_to_string(cli.configuration)?.as_str())?;
    println!("{:?}", config);
    
    let mut ffmpeg = Command::new("ffmpeg")
        .stdout(Stdio::piped())
        .args(config.to_vec())
        .kill_on_drop(true)
        .spawn()?;
    
    println!("{:?}", ffmpeg);
    //let stdout = ffmpeg.stdout.take().unwrap();

    _ = ffmpeg.wait().await?;
    println!("process exited");
    Ok(())
}
