use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::medicine_reminder::{MedicineReminder, UpdateMedicineReminderRequest},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(db: &PgPool, reminder: &MedicineReminder) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO medicine_reminders
         (id, cat_id, owner_id, reminder_type, label, scheduled_date,
          is_recurring, interval_days, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(reminder.id)
    .bind(reminder.cat_id)
    .bind(reminder.owner_id)
    .bind(&reminder.reminder_type)
    .bind(&reminder.label)
    .bind(reminder.scheduled_date)
    .bind(reminder.is_recurring)
    .bind(reminder.interval_days)
    .bind(reminder.is_active)
    .bind(reminder.created_at)
    .bind(reminder.updated_at)
    .execute(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: &Uuid) -> Result<Option<MedicineReminder>, AppError> {
    sqlx::query_as::<_, MedicineReminder>(
        "SELECT id, cat_id, owner_id, reminder_type, label, scheduled_date,
                is_recurring, interval_days, is_active, created_at, updated_at
         FROM medicine_reminders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn list_by_cat(
    db: &PgPool,
    cat_id: &Uuid,
    owner_id: &Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<MedicineReminder>, AppError> {
    sqlx::query_as::<_, MedicineReminder>(
        "SELECT id, cat_id, owner_id, reminder_type, label, scheduled_date,
                is_recurring, interval_days, is_active, created_at, updated_at
         FROM medicine_reminders
         WHERE cat_id = $1 AND owner_id = $2
         ORDER BY scheduled_date ASC
         LIMIT $3 OFFSET $4",
    )
    .bind(cat_id)
    .bind(owner_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn update(
    db: &PgPool,
    id: &Uuid,
    owner_id: &Uuid,
    req: &UpdateMedicineReminderRequest,
) -> Result<MedicineReminder, AppError> {
    let existing = find_by_id(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("medicine reminder not found".to_string()))?;

    if existing.owner_id != *owner_id {
        return Err(AppError::Forbidden);
    }

    let reminder_type = req
        .reminder_type
        .as_deref()
        .unwrap_or(&existing.reminder_type)
        .to_string();
    let label = req.label.as_deref().unwrap_or(&existing.label).to_string();
    let scheduled_date: DateTime<Utc> = req
        .scheduled_date
        .as_deref()
        .map(parse_datetime)
        .transpose()?
        .unwrap_or(existing.scheduled_date);
    let is_recurring = req.is_recurring.unwrap_or(existing.is_recurring);
    let interval_days: Option<i32> = if req.interval_days.is_some() {
        req.interval_days
    } else {
        existing.interval_days
    };
    let is_active = req.is_active.unwrap_or(existing.is_active);
    let now = Utc::now();

    sqlx::query_as::<_, MedicineReminder>(
        "UPDATE medicine_reminders
         SET reminder_type = $1, label = $2, scheduled_date = $3,
             is_recurring = $4, interval_days = $5, is_active = $6, updated_at = $7
         WHERE id = $8 AND owner_id = $9
         RETURNING id, cat_id, owner_id, reminder_type, label, scheduled_date,
                   is_recurring, interval_days, is_active, created_at, updated_at",
    )
    .bind(&reminder_type)
    .bind(&label)
    .bind(scheduled_date)
    .bind(is_recurring)
    .bind(interval_days)
    .bind(is_active)
    .bind(now)
    .bind(id)
    .bind(owner_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn delete(db: &PgPool, id: &Uuid, owner_id: &Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM medicine_reminders WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "medicine reminder not found".to_string(),
        ));
    }
    Ok(())
}

fn parse_datetime(s: &str) -> Result<DateTime<Utc>, AppError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| AppError::BadRequest("invalid scheduledDate; expected ISO 8601".to_string()))
}
