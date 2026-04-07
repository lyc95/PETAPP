use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::Utc;
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
) -> Result<Json<ApiList<HealthRecord>>, AppError> {
    let records = health_repo::list_by_cat(
        &state.dynamo,
        &state.config.health_records_table,
        &cat_id,
        &auth.sub,
    )
    .await?;
    Ok(Json(ApiList::ok(records)))
}

async fn create_health_record(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Json(req): Json<CreateHealthRecordRequest>,
) -> Result<(StatusCode, Json<ApiResponse<HealthRecord>>), AppError> {
    let recorded_at = parse_datetime(&req.recorded_at)?;
    let now = Utc::now();
    let record = HealthRecord {
        id: Uuid::new_v4(),
        cat_id,
        owner_id: auth.sub,
        record_type: req.record_type,
        title: req.title,
        description: req.description,
        recorded_at,
        attachment_key: req.attachment_key,
        created_at: now,
        updated_at: now,
    };
    health_repo::create(&state.dynamo, &state.config.health_records_table, &record).await?;
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
    let record = health_repo::update(
        &state.dynamo,
        &state.config.health_records_table,
        &id,
        &auth.sub,
        &req,
    )
    .await?;
    Ok(Json(ApiResponse::ok(record)))
}

async fn delete_health_record(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    health_repo::delete(
        &state.dynamo,
        &state.config.health_records_table,
        &id,
        &auth.sub,
    )
    .await?;
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
        assert!(parse_datetime("2026-03-10T14:00:00Z").is_ok());
    }

    #[test]
    fn date_only_rejected() {
        assert!(parse_datetime("2026-03-10").is_err());
    }
}
