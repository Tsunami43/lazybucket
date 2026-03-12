use axum::extract::{Path, State};

use crate::{AppState, db};

pub async fn create_bucket(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> &'static str {
    db::buckets::create_bucket(&state.pool, &name)
        .await
        .unwrap();
    "ok"
}
