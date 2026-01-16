use crate::AppState;
use crate::command::CommandAction;
use crate::ui::UI_ROOT;
use crate::ui::webpage::index_page;
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use maud::Markup;
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;

pub async fn display_index(State(state): State<Arc<AppState>>) -> Result<Markup, StatusCode> {
    Ok(index_page(state).await)
}

pub async fn post_command(
    State(state): State<Arc<AppState>>,
    Form(form): Form<Vec<(String, String)>>,
) -> Result<impl IntoResponse, StatusCode> {
    let (key, _) = form.first().unwrap();
    let cmd = CommandAction::from_str(key.as_str()).expect("");
    debug!("{:?}", cmd);
    state
        .sender
        .send(cmd)
        .await
        .expect("unable to send on channel");

    Ok(Redirect::to(UI_ROOT))
}
