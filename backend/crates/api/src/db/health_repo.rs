use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::health_record::{HealthRecord, UpdateHealthRecordRequest},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(db: &PgPool, record: &HealthRecord) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO health_records
         (id, cat_id, owner_id, record_type, title, description, recorded_at, attachment_key, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(record.id)
    .bind(record.cat_id)
    .bind(record.owner_id)
    .bind(&record.record_type)
    .bind(&record.title)
    .bind(&record.description)
    .bind(record.recorded_at)
    .bind(&record.attachment_key)
    .bind(record.created_at)
    .bind(record.updated_at)
    .execute(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: &Uuid) -> Result<Option<HealthRecord>, AppError> {
    sqlx::query_as::<_, HealthRecord>(
        "SELECT id, cat_id, owner_id, record_type, title, description,
                recorded_at, attachment_key, created_at, updated_at
         FROM health_records WHERE id = $1",
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
) -> Result<Vec<HealthRecord>, AppError> {
    sqlx::query_as::<_, HealthRecord>(
        "SELECT id, cat_id, owner_id, record_type, title, description,
                recorded_at, attachment_key, created_at, updated_at
         FROM health_records
         WHERE cat_id = $1 AND owner_id = $2
         ORDER BY recorded_at DESC
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
    req: &UpdateHealthRecordRequest,
) -> Result<HealthRecord, AppError> {
    let existing = find_by_id(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("health record not found".to_string()))?;

    if existing.owner_id != *owner_id {
        return Err(AppError::Forbidden);
    }

    let record_type = req
        .record_type
        .as_deref()
        .unwrap_or(&existing.record_type)
        .to_string();
    let title = req.title.as_deref().unwrap_or(&existing.title).to_string();
    let description = req
        .description
        .as_deref()
        .unwrap_or(&existing.description)
        .to_string();
    let recorded_at = req
        .recorded_at
        .as_deref()
        .map(parse_datetime)
        .transpose()?
        .unwrap_or(existing.recorded_at);
    let attachment_key: Option<String> = if req.attachment_key.is_some() {
        req.attachment_key.clone()
    } else {
        existing.attachment_key.clone()
    };
    let now = Utc::now();

    sqlx::query_as::<_, HealthRecord>(
        "UPDATE health_records
         SET record_type = $1, title = $2, description = $3,
             recorded_at = $4, attachment_key = $5, updated_at = $6
         WHERE id = $7 AND owner_id = $8
         RETURNING id, cat_id, owner_id, record_type, title, description,
                   recorded_at, attachment_key, created_at, updated_at",
    )
    .bind(&record_type)
    .bind(&title)
    .bind(&description)
    .bind(recorded_at)
    .bind(&attachment_key)
    .bind(now)
    .bind(id)
    .bind(owner_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn delete(db: &PgPool, id: &Uuid, owner_id: &Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM health_records WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("health record not found".to_string()));
    }
    Ok(())
}

fn parse_datetime(s: &str) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|_| AppError::BadRequest("invalid datetime; expected ISO 8601".to_string()))
}
