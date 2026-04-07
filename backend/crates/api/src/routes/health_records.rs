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
    db::health_repo,
    errors::AppError,
    models::{
        api_response::{ApiList, ApiResponse},
        health_record::{CreateHealthRecordRequest, HealthRecord, UpdateHealthRecordRequest},
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
            "/cats/:catId/health-records",
            get(list_health_records).post(create_health_record),
        )
        .route(
            "/health-records/:id",
            axum::routing::patch(update_health_record).delete(delete_health_record),
        )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list_health_records(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ApiList<HealthRecord>>, AppError> {
    let limit = pagination.limit.unwrap_or(DEFAULT_LIMIT).min(200);
    let offset = pagination.offset.unwrap_or(0).max(0);
    let records = health_repo::list_by_cat(&state.db, &cat_id, &auth.id, limit, offset).await?;
    Ok(Json(ApiList::new(records)))
}

async fn create_health_record(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Json(req): Json<CreateHealthRecordRequest>,
) -> Result<(StatusCode, Json<ApiResponse<HealthRecord>>), AppError> {
    require_non_empty(&req.title, "title")?;
    require_non_empty(&req.record_type, "recordType")?;
    let recorded_at = parse_datetime(&req.recorded_at)?;
    let now = Utc::now();
    let record = HealthRecord {
        id: Uuid::new_v4(),
        cat_id,
        owner_id: auth.id,
        record_type: req.record_type,
        title: req.title,
        description: req.description,
        recorded_at,
        attachment_key: req.attachment_key,
        created_at: now,
        updated_at: now,
    };
    health_repo::create(&state.db, &record).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(record))))
}

async fn update_health_record(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateHealthRecordRequest>,
) -> Result<Json<ApiResponse<HealthRecord>>, AppError> {
    if let Some(ra) = &req.recorded_at {
        parse_datetime(ra)?;
    }
    let record = health_repo::update(&state.db, &id, &auth.id, &req).await?;
    Ok(Json(ApiResponse::ok(record)))
}

async fn delete_health_record(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    health_repo::delete(&state.db, &id, &auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn require_non_empty(s: &str, field: &str) -> Result<(), AppError> {
    if s.trim().is_empty() {
        Err(AppError::BadRequest(format!("{field} must not be empty")))
    } else {
        Ok(())
    }
}

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
        assert!(parse_datetime("2026-03-10T14:00:00Z").is_ok());
    }

    #[test]
    fn date_only_rejected() {
        assert!(parse_datetime("2026-03-10").is_err());
    }
}
