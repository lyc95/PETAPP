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
    db::medicine_repo,
    errors::AppError,
    models::{
        api_response::{ApiList, ApiResponse},
        medicine_reminder::{
            CreateMedicineReminderRequest, MedicineReminder, UpdateMedicineReminderRequest,
        },
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
            "/cats/:catId/medicine-reminders",
            get(list_medicine_reminders).post(create_medicine_reminder),
        )
        .route(
            "/medicine-reminders/:id",
            axum::routing::patch(update_medicine_reminder).delete(delete_medicine_reminder),
        )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list_medicine_reminders(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ApiList<MedicineReminder>>, AppError> {
    let limit = pagination.limit.unwrap_or(DEFAULT_LIMIT).min(200);
    let offset = pagination.offset.unwrap_or(0).max(0);
    let reminders = medicine_repo::list_by_cat(&state.db, &cat_id, &auth.id, limit, offset).await?;
    Ok(Json(ApiList::new(reminders)))
}

async fn create_medicine_reminder(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Json(req): Json<CreateMedicineReminderRequest>,
) -> Result<(StatusCode, Json<ApiResponse<MedicineReminder>>), AppError> {
    require_non_empty(&req.label, "label")?;
    require_non_empty(&req.reminder_type, "reminderType")?;
    let scheduled_date = parse_datetime(&req.scheduled_date)?;
    let now = Utc::now();
    let reminder = MedicineReminder {
        id: Uuid::new_v4(),
        cat_id,
        owner_id: auth.id,
        reminder_type: req.reminder_type,
        label: req.label,
        scheduled_date,
        is_recurring: req.is_recurring,
        interval_days: req.interval_days,
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    medicine_repo::create(&state.db, &reminder).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(reminder))))
}

async fn update_medicine_reminder(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMedicineReminderRequest>,
) -> Result<Json<ApiResponse<MedicineReminder>>, AppError> {
    if let Some(date) = &req.scheduled_date {
        parse_datetime(date)?;
    }
    let reminder = medicine_repo::update(&state.db, &id, &auth.id, &req).await?;
    Ok(Json(ApiResponse::ok(reminder)))
}

async fn delete_medicine_reminder(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    medicine_repo::delete(&state.db, &id, &auth.id).await?;
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

fn parse_datetime(s: &str) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|_| {
            AppError::BadRequest("invalid scheduledDate; expected ISO 8601 datetime".to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_scheduled_date_accepted() {
        assert!(parse_datetime("2026-05-01T10:00:00Z").is_ok());
    }

    #[test]
    fn date_only_rejected() {
        assert!(parse_datetime("2026-05-01").is_err());
    }

    #[test]
    fn free_text_rejected() {
        assert!(parse_datetime("next monday").is_err());
    }
}
