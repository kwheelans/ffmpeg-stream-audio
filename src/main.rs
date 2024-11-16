mod cli;
mod configuration;
mod error;

use crate::cli::CliArgs;
use crate::configuration::{CommandConfig, StreamConfig};
use crate::error::Error;
use clap::Parser;
use std::process::{ExitCode, Stdio};
use tokio::process::Command;
use tracing::level_filters::LevelFilter;
use tracing::{debug, error, info};

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let cli = CliArgs::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level.unwrap_or(LevelFilter::INFO))
        .init();

    match run(cli).await {
        Err(error) => {
            error!("{}", error);
            ExitCode::FAILURE
        }
        Ok(code) => match code {
            None => ExitCode::FAILURE,
            Some(n) => ExitCode::from(n as u8),
        },
    }
}

async fn run(cli: CliArgs) -> Result<Option<i32>, Error> {
    let config = StreamConfig::try_from(std::fs::read_to_string(cli.configuration)?.as_str())?;
    debug!("{:?}", config);

    let mut ffmpeg = Command::new("ffmpeg")
        .stdout(Stdio::piped())
        .args(config.to_vec())
        .kill_on_drop(true)
        .spawn()?;

    debug!("{:?}", ffmpeg);

    let exit = tokio::select! {
        process = ffmpeg.wait() => process?.code(),
        exit = listen_for_shutdown() => Some(exit),
    };

    Ok(exit)
}

#[cfg(unix)]
async fn listen_for_shutdown() -> i32 {
    use tokio::signal::unix::{signal, SignalKind};
    // Listen for SIGTERM and SIGINT to know when shutdown
    let mut sigterm =
        signal(SignalKind::terminate()).expect("unable to listen for terminate signal");
    let mut sigint =
        signal(SignalKind::interrupt()).expect("unable to listen for interrupt signal");

    let exit = tokio::select! {
        _ = sigterm.recv() => {
            debug!("Received SIGTERM.");
            SignalKind::terminate().as_raw_value()
        },
        _ = sigint.recv() => {
            debug!("Received SIGINT.");
            SignalKind::interrupt().as_raw_value()
        },
    };

    info!("Shutting Down");
    exit
}

#[cfg(windows)]
async fn listen_for_shutdown() -> i32 {
    use tokio::signal::windows::{ctrl_break, ctrl_c};
    // Listen for CTRL-C and CTRL-BREAK to know when shutdown
    let mut sig_ctrl_break = ctrl_break().expect("unable to listen for ctrl-break signal");
    let mut sig_ctrl_c = ctrl_c().expect("unable to listen for ctrl-c signal");

    let exit = tokio::select! {
        _ = sig_ctrl_break.recv() => {
            debug!("Received CTRL-BREAK.");
            1
        },
        _ = sig_ctrl_c.recv() => {
            debug!("Received CTRL-C.");
            1
        },
    };

    info!("Shutting Down");
    exit
}
