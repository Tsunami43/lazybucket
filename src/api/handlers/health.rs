use axum::extract::State;

use crate::AppState;

pub async fn health(_state: State<AppState>) -> &'static str {
    "ok"
}
