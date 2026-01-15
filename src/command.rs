use crate::AppState;
use crate::command::CommandAction::{Restart, Start, Stop};
use chrono::Local;
use std::ffi::OsString;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use strum::{Display, EnumString};
use tokio::process::{Child, Command};
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

#[derive(Debug, Copy, Clone, Display, EnumString)]
pub enum CommandAction {
    Start,
    Stop,
    Restart,
}

pub async fn command_orchestration(mut rx: Receiver<CommandAction>, state: Arc<AppState>) {
    while let Some(action) = rx.recv().await {
        let state = state.clone();
        match action {
            Start => start_command(state).await,
            Stop => stop_command(state).await,
            Restart => restart_command(state).await,
        }
    }
}

async fn start_command(state: Arc<AppState>) {
    info!("Action: Start");
    let mut task_status = state.task_status.lock().await;
    match task_status.handle.as_ref() {
        Some(handle) if !handle.is_finished() => {
            warn!("Task is already running")
        }
        _ => task_status.handle = Some(start_task(state.stream_config.clone())),
    }
}

async fn stop_command(state: Arc<AppState>) {
    info!("Action: Stop");
    let mut task_status = state.task_status.lock().await;
    match task_status.handle.as_ref() {
        Some(handle) if !handle.is_finished() => {
            info!("Stopping task");
            if let Some(h) = task_status.handle.as_ref() {
                h.abort();
                task_status.handle = None;
                task_status.message = "Task Stopped".into();
                task_status.timestamp = Local::now();
            }
        }

        _ => info!("No task running"),
    }
}

async fn restart_command(state: Arc<AppState>) {
    info!("Action: Restart");
    stop_command(state.clone()).await;
    tokio::time::sleep(Duration::from_micros(1)).await;
    start_command(state.clone()).await;
}

fn start_task(config: Vec<OsString>) -> JoinHandle<Option<i32>> {
    tokio::spawn(async {
        match ffmpeg_cmd(config) {
            Ok(mut p) => match p.wait().await {
                Ok(s) => s.code(),
                Err(e) => {
                    error!("Error on wait: {}", e);
                    Some(2)
                }
            },
            Err(e) => {
                error!("Unable to spawn command: {}", e);
                Some(2)
            }
        }
    })
}

fn ffmpeg_cmd(config: Vec<OsString>) -> std::io::Result<Child> {
    Command::new("ffmpeg")
        .stdout(Stdio::piped())
        .args(config)
        .kill_on_drop(true)
        .spawn()
}
