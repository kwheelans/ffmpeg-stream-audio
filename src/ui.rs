use crate::AppState;
use crate::error::Error;
use crate::ui::handlers::{display_index, post_command};
use axum::routing::get;
use axum::{Router, serve};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

mod handlers;
mod webpage;

const UI_ROOT: &str = "/";

pub async fn serve_ui(state: Arc<AppState>, listener: TcpListener) -> Result<(), Error> {
    let routes = Router::new()
        .route(UI_ROOT, get(display_index).post(post_command))
        .with_state(state);

    info!("Listening on {}", listener.local_addr()?);
    Ok(serve(listener, routes).await?)
}
