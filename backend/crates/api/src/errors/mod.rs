use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// Variants are used progressively across phases.
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal error")]
    Internal(#[source] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let message = self.to_string();
        (
            status,
            Json(json!({ "error": { "code": code, "message": message } })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status(err: AppError) -> StatusCode {
        err.into_response().status()
    }

    #[test]
    fn unauthorized_returns_401() {
        assert_eq!(status(AppError::Unauthorized), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn forbidden_returns_403() {
        assert_eq!(status(AppError::Forbidden), StatusCode::FORBIDDEN);
    }

    #[test]
    fn not_found_returns_404() {
        assert_eq!(
            status(AppError::NotFound("cat not found".to_string())),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn bad_request_returns_400() {
        assert_eq!(
            status(AppError::BadRequest("invalid date".to_string())),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn internal_returns_500() {
        assert_eq!(
            status(AppError::Internal(anyhow::anyhow!("db error"))),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
