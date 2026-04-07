use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::state::AppState;

/// The authenticated user extracted from a valid JWT.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user UUID
    exp: i64,
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let token = match extract_bearer(&req) {
        Some(t) => t,
        None => return unauthorized("missing or invalid Authorization header"),
    };

    let key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&["catcare"]);
    validation.set_audience(&["catcare"]);

    match decode::<Claims>(token, &key, &validation) {
        Ok(data) => {
            let id = match Uuid::parse_str(&data.claims.sub) {
                Ok(u) => u,
                Err(_) => return unauthorized("invalid token subject"),
            };
            req.extensions_mut().insert(AuthUser { id });
            next.run(req).await
        }
        Err(_) => unauthorized("invalid or expired token"),
    }
}

fn extract_bearer(req: &Request<Body>) -> Option<&str> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

fn unauthorized(message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": { "code": "UNAUTHORIZED", "message": message } })),
    )
        .into_response()
}
