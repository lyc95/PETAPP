use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    db::weight_repo,
    errors::AppError,
    models::{
        api_response::{ApiList, ApiResponse},
        weight_log::{CreateWeightLogRequest, UpdateWeightLogRequest, WeightLog},
    },
    state::AppState,
};

const DEFAULT_LIMIT: i64 = 50;

#[derive(Deserialize)]
struct Pagination {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/cats/:catId/weight-logs",
            get(list_weight_logs).post(create_weight_log),
        )
        .route(
            "/weight-logs/:id",
            axum::routing::patch(update_weight_log).delete(delete_weight_log),
        )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list_weight_logs(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ApiList<WeightLog>>, AppError> {
    let limit = pagination.limit.unwrap_or(DEFAULT_LIMIT).min(200);
    let offset = pagination.offset.unwrap_or(0).max(0);
    let logs = weight_repo::list_by_cat(&state.db, &cat_id, &auth.id, limit, offset).await?;
    Ok(Json(ApiList::new(logs)))
}

async fn create_weight_log(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Json(req): Json<CreateWeightLogRequest>,
) -> Result<(StatusCode, Json<ApiResponse<WeightLog>>), AppError> {
    if req.weight_kg <= 0.0 {
        return Err(AppError::BadRequest(
            "weightKg must be positive".to_string(),
        ));
    }
    let logged_at = parse_datetime(&req.logged_at)?;
    let now = Utc::now();
    let log = WeightLog {
        id: Uuid::new_v4(),
        cat_id,
        owner_id: auth.id,
        weight_kg: req.weight_kg,
        logged_at,
        note: req.note,
        created_at: now,
        updated_at: now,
    };
    weight_repo::create(&state.db, &log).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(log))))
}

async fn update_weight_log(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateWeightLogRequest>,
) -> Result<Json<ApiResponse<WeightLog>>, AppError> {
    if let Some(kg) = req.weight_kg {
        if kg <= 0.0 {
            return Err(AppError::BadRequest(
                "weightKg must be positive".to_string(),
            ));
        }
    }
    if let Some(la) = &req.logged_at {
        parse_datetime(la)?;
    }
    let log = weight_repo::update(&state.db, &id, &auth.id, &req).await?;
    Ok(Json(ApiResponse::ok(log)))
}

async fn delete_weight_log(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    weight_repo::delete(&state.db, &id, &auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_datetime(s: &str) -> Result<chrono::DateTime<Utc>, AppError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| AppError::BadRequest("invalid datetime; expected ISO 8601".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_datetime_accepted() {
        assert!(parse_datetime("2026-04-01T10:00:00Z").is_ok());
    }

    #[test]
    fn date_only_rejected() {
        assert!(parse_datetime("2026-04-01").is_err());
    }
}
