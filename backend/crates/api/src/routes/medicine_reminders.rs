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
) -> Result<Json<ApiList<MedicineReminder>>, AppError> {
    let reminders = medicine_repo::list_by_cat(
        &state.dynamo,
        &state.config.medicine_reminders_table,
        &cat_id,
        &auth.sub,
    )
    .await?;
    Ok(Json(ApiList::ok(reminders)))
}

async fn create_medicine_reminder(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Json(req): Json<CreateMedicineReminderRequest>,
) -> Result<(StatusCode, Json<ApiResponse<MedicineReminder>>), AppError> {
    parse_scheduled_date(&req.scheduled_date)?;

    let now = Utc::now();
    let reminder = MedicineReminder {
        id: Uuid::new_v4(),
        cat_id,
        owner_id: auth.sub,
        reminder_type: req.reminder_type,
        label: req.label,
        scheduled_date: req.scheduled_date,
        is_recurring: req.is_recurring,
        interval_days: req.interval_days,
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    medicine_repo::create(
        &state.dynamo,
        &state.config.medicine_reminders_table,
        &reminder,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(reminder))))
}

async fn update_medicine_reminder(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMedicineReminderRequest>,
) -> Result<Json<ApiResponse<MedicineReminder>>, AppError> {
    if let Some(date) = &req.scheduled_date {
        parse_scheduled_date(date)?;
    }
    let reminder = medicine_repo::update(
        &state.dynamo,
        &state.config.medicine_reminders_table,
        &id,
        &auth.sub,
        &req,
    )
    .await?;
    Ok(Json(ApiResponse::ok(reminder)))
}

async fn delete_medicine_reminder(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    medicine_repo::delete(
        &state.dynamo,
        &state.config.medicine_reminders_table,
        &id,
        &auth.sub,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_scheduled_date(s: &str) -> Result<(), AppError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|_| ())
        .map_err(|_| {
            AppError::BadRequest("invalid scheduledDate; expected ISO 8601 datetime".to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_scheduled_date_accepted() {
        assert!(parse_scheduled_date("2026-05-01T10:00:00Z").is_ok());
    }

    #[test]
    fn date_only_rejected() {
        assert!(parse_scheduled_date("2026-05-01").is_err());
    }

    #[test]
    fn free_text_rejected() {
        assert!(parse_scheduled_date("next monday").is_err());
    }
}
