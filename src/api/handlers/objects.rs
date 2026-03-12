use axum::{
    Json,
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{AppState, db};

pub async fn upload_object(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> Result<StatusCode, StatusCode> {
    let etag = hex::encode(Sha256::digest(&body));
    let size = body.len() as i64;
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    state
        .storage
        .write(&bucket, &key, body.to_vec())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let obj = db::objects::Object {
        bucket: bucket.clone(),
        key: key.clone(),
        size,
        content_type,
        etag,
        storage_path: format!("{}/{}", bucket, key),
    };

    db::objects::create_object(&state.pool, &obj)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("Object uploaded: {}/{}", bucket, key);
    Ok(StatusCode::CREATED)
}

pub async fn download_object(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<Response, StatusCode> {
    let obj = db::objects::get_object(&state.pool, &bucket, &key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let stream = state
        .storage
        .read_stream(&bucket, &key)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let content_type = obj
        .content_type
        .unwrap_or_else(|| "application/octet-stream".to_string());

    Ok((
        [(header::CONTENT_TYPE, content_type)],
        Body::from_stream(stream),
    )
        .into_response())
}

pub async fn delete_object(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    let deleted = db::objects::delete_object(&state.pool, &bucket, &key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !deleted {
        return Err(StatusCode::NOT_FOUND);
    }

    state
        .storage
        .delete(&bucket, &key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("Object deleted: {}/{}", bucket, key);
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct RenameObjectRequest {
    pub key: String,
}

pub async fn rename_object(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
    Json(body): Json<RenameObjectRequest>,
) -> Result<StatusCode, StatusCode> {
    let renamed = db::objects::rename_object(&state.pool, &bucket, &key, &body.key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !renamed {
        return Err(StatusCode::NOT_FOUND);
    }

    state
        .storage
        .rename(&bucket, &key, &body.key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("Object renamed: {}/{} -> {}", bucket, key, body.key);
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub prefix: Option<String>,
}

#[derive(Serialize)]
pub struct ObjectInfo {
    pub key: String,
    pub size: i64,
    pub content_type: Option<String>,
    pub etag: String,
}

pub async fn list_objects(
    State(state): State<AppState>,
    Path(bucket): Path<String>,
    Query(params): Query<ListQuery>,
) -> Result<Json<Vec<ObjectInfo>>, StatusCode> {
    let objects = db::objects::list_objects(&state.pool, &bucket, params.prefix.as_deref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = objects
        .into_iter()
        .map(|o| ObjectInfo {
            key: o.key,
            size: o.size,
            content_type: o.content_type,
            etag: o.etag,
        })
        .collect();

    Ok(Json(result))
}
