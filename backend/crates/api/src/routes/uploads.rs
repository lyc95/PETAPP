use std::{path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::{Extension, Multipart, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tokio::fs;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{auth::AuthUser, errors::AppError, models::api_response::ApiResponse, state::AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/uploads/file", post(upload_file))
        .route("/files/*key", get(serve_file))
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Accept a multipart file upload, save to disk, return the object key.
async fn upload_file(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("multipart error: {e}")))?
        .ok_or_else(|| AppError::BadRequest("no file field in request".to_string()))?;

    let file_name = field.file_name().unwrap_or("upload").to_string();
    let ext = std::path::Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");

    let object_key = format!("{}/{}.{}", auth.id, Uuid::new_v4(), ext);
    let file_path = PathBuf::from(&state.config.upload_dir).join(&object_key);

    // Ensure owner directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("create dir: {e}")))?;
    }

    let bytes = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("read file: {e}")))?;

    fs::write(&file_path, &bytes)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("write file: {e}")))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::ok(
            serde_json::json!({ "objectKey": object_key }),
        )),
    ))
}

/// Stream a file from disk to the client.
async fn serve_file(
    Extension(_auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    // Prevent path traversal
    if key.contains("..") {
        return Err(AppError::BadRequest("invalid key".to_string()));
    }

    let file_path = PathBuf::from(&state.config.upload_dir).join(&key);
    let file = fs::File::open(&file_path)
        .await
        .map_err(|_| AppError::NotFound("file not found".to_string()))?;

    let mime = mime_guess(&key);
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(([(header::CONTENT_TYPE, mime)], body).into_response())
}

fn mime_guess(key: &str) -> &'static str {
    match key.rsplit('.').next().unwrap_or("") {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        _ => "application/octet-stream",
    }
}
