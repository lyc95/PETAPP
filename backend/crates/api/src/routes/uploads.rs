use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    auth::AuthUser, errors::AppError, models::api_response::ApiResponse, s3, state::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/uploads/presign", post(presign_upload))
        .route("/files/:encodedKey/url", get(get_file_url))
}

// ---------------------------------------------------------------------------
// Request / response types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PresignRequest {
    #[allow(dead_code)]
    file_name: String,
    content_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PresignResponse {
    upload_url: String,
    object_key: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn presign_upload(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<PresignRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PresignResponse>>), AppError> {
    let object_key = format!("photos/{}/{}", auth.sub, Uuid::new_v4());
    let upload_url = s3::presign_put(
        &state.s3,
        &state.config.s3_bucket,
        &object_key,
        &req.content_type,
        Duration::from_secs(300), // 5 min TTL
    )
    .await
    .map_err(AppError::Internal)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::ok(PresignResponse {
            upload_url,
            object_key,
        })),
    ))
}

async fn get_file_url(
    Extension(_auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(encoded_key): Path<String>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let key_bytes = URL_SAFE_NO_PAD
        .decode(&encoded_key)
        .map_err(|_| AppError::BadRequest("invalid key encoding".to_string()))?;

    let key = String::from_utf8(key_bytes)
        .map_err(|_| AppError::BadRequest("key is not valid UTF-8".to_string()))?;

    let download_url = s3::presign_get(
        &state.s3,
        &state.config.s3_bucket,
        &key,
        Duration::from_secs(900), // 15 min TTL
    )
    .await
    .map_err(AppError::Internal)?;

    Ok(Json(ApiResponse::ok(
        json!({ "downloadUrl": download_url }),
    )))
}
