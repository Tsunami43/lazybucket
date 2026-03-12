use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{AppState, db};

pub async fn create_bucket(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match db::buckets::create_bucket(&state.pool, &name).await {
        Ok(_) => {
            tracing::info!("Bucket created: {}", name);
            Ok(StatusCode::CREATED)
        }
        Err(_) => {
            tracing::warn!("Bucket already exists: {}", name);
            Err(StatusCode::CONFLICT)
        }
    }
}
