mod cli;
mod command;
mod configuration;
mod error;
mod ui;

use crate::cli::CliArgs;
use crate::command::{CommandAction, command_orchestration};
use crate::configuration::{CommandConfig, Configuration};
use crate::error::Error;
use crate::ui::serve_ui;
use chrono::{DateTime, Local};
use clap::Parser;
use std::ffi::OsString;
use std::process::ExitCode;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;
use tracing::level_filters::LevelFilter;
use tracing::{debug, error, info};

#[derive(Debug)]
struct AppState {
    task_status: Mutex<TaskStatus>,
    sender: Sender<CommandAction>,
    ffmpeg_config: Vec<OsString>,
    css: String,
}

#[derive(Debug, Default)]
struct TaskStatus {
    handle: Option<JoinHandle<Option<i32>>>,
    message: String,
    timestamp: DateTime<Local>,
}

impl TaskStatus {
    pub fn running(&self) -> bool {
        matches!(self.handle.as_ref(), Some(handle) if !handle.is_finished())
    }
}

#[tokio::main()]
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
    let config = Configuration::try_from(std::fs::read_to_string(cli.configuration)?.as_str())?;
    debug!("{:?}", config);

    // Initialize channel
    let (sender, receiver) = mpsc::channel(5);

    // Initialize state
    let state = Arc::new(AppState {
        task_status: Mutex::new(TaskStatus::default()),
        sender,
        ffmpeg_config: config.ffmpeg().to_vec(),
        css: config.ui().get_stylesheet_href(),
    });

    // Setup address to listen on
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.ui().listen_address(),
        config.ui().port()
    ))
    .await?;

    // Start waiting for commands from UI
    tokio::spawn(command_orchestration(receiver, state.clone()));

    // Start UI and listen for kill signals
    let exit = tokio::select! {
        ui = serve_ui(state.clone(), listener) => {
            match ui {
                Ok(_) => Some(0),
                Err(e) => {
                    error!("Error serving ui: {}", e);
                    Some(1)
                }
            }
        },
        exit = listen_for_shutdown() => Some(exit),
    };

    Ok(exit)
}

#[cfg(unix)]
async fn listen_for_shutdown() -> i32 {
    use tokio::signal::unix::{SignalKind, signal};
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
