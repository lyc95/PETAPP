use std::sync::Arc;

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

use crate::{
    auth::{service, AuthUser},
    db::users_repo,
    errors::AppError,
    models::{api_response::ApiResponse, user::AuthRequest},
    state::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
}

// ---------------------------------------------------------------------------
// Response type
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AuthRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TokenResponse>>), AppError> {
    if req.email.trim().is_empty() {
        return Err(AppError::BadRequest("email must not be empty".to_string()));
    }
    if req.password.len() < 8 {
        return Err(AppError::BadRequest(
            "password must be at least 8 characters".to_string(),
        ));
    }

    let hash = service::hash_password(&req.password)?;
    let user = users_repo::create(&state.db, &req.email, &hash).await?;
    let token = service::issue_token(user.id, &state.config.jwt_secret)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::ok(TokenResponse { token })),
    ))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, AppError> {
    let row = users_repo::find_by_email(&state.db, &req.email)
        .await?
        .ok_or_else(|| AppError::BadRequest("invalid email or password".to_string()))?;

    service::verify_password(&req.password, &row.password_hash)?;
    let token = service::issue_token(row.id, &state.config.jwt_secret)?;

    Ok(Json(ApiResponse::ok(TokenResponse { token })))
}

async fn me(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<crate::models::user::User>>, AppError> {
    let user = users_repo::find_by_id(&state.db, &auth.id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;
    Ok(Json(ApiResponse::ok(user)))
}
