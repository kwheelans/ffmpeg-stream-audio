use crate::AppState;
use crate::command::CommandAction;
use maud::{DOCTYPE, Markup, html};
use std::sync::Arc;

const PAGE_TITLE: &str = "FFMPEG Stream Audio";

async fn base(css_path: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light dark";
                link rel="stylesheet" href=(css_path);
            }
        }
    }
}

async fn title(title: &str) -> Markup {
    html! {
        header {
            h1 { (title) };
            hr;
        }
    }
}

pub async fn index_page(state: Arc<AppState>) -> Markup {
    let task_status = state.task_status.lock().await;
    html! {
        (base(state.css.as_str()).await)
        body {
            (title(PAGE_TITLE).await)
            main {
                table {
                    thead {
                        td { "Task Status" }
                        td { "Timestamp "}
                    }
                    tbody {
                        tr {
                            td { (task_status.status()) }
                            td { (task_status.timestamp()) }
                        }
                    }
                }
                form action={ "/" } method="post" {
                    button type="submit" name=(CommandAction::Start) { (CommandAction::Start) }
                    button type="submit" name=(CommandAction::Stop) { (CommandAction::Stop) }
                    button type="submit" name=(CommandAction::Restart) { (CommandAction::Restart) }
                }

            }
        }
    }
}
