use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{AppState, db};

#[derive(Deserialize)]
pub struct PatchBucketRequest {
    pub name: String,
}

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

#[derive(Serialize)]
pub struct BucketInfo {
    pub name: String,
    pub created_at: String,
}

pub async fn list_buckets(State(state): State<AppState>) -> Result<Json<Vec<BucketInfo>>, StatusCode> {
    match db::buckets::list_buckets(&state.pool).await {
        Ok(buckets) => Ok(Json(buckets.into_iter().map(|b| BucketInfo {
            name: b.name,
            created_at: b.created_at,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn rename_bucket(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(body): Json<PatchBucketRequest>,
) -> Result<StatusCode, StatusCode> {
    match db::buckets::rename_bucket(&state.pool, &name, &body.name).await {
        Ok(true) => {
            tracing::info!("Bucket renamed: {} -> {}", name, body.name);
            Ok(StatusCode::OK)
        }
        Ok(false) => {
            tracing::warn!("Bucket not found: {}", name);
            Err(StatusCode::NOT_FOUND)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_bucket(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match db::buckets::delete_bucket(&state.pool, &name).await {
        Ok(true) => {
            tracing::info!("Bucket deleted: {}", name);
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => {
            tracing::warn!("Bucket not found: {}", name);
            Err(StatusCode::NOT_FOUND)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
