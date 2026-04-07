use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::weight_log::{UpdateWeightLogRequest, WeightLog},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(db: &PgPool, log: &WeightLog) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO weight_logs (id, cat_id, owner_id, weight_kg, logged_at, note, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(log.id)
    .bind(log.cat_id)
    .bind(log.owner_id)
    .bind(log.weight_kg)
    .bind(log.logged_at)
    .bind(&log.note)
    .bind(log.created_at)
    .bind(log.updated_at)
    .execute(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: &Uuid) -> Result<Option<WeightLog>, AppError> {
    sqlx::query_as::<_, WeightLog>(
        "SELECT id, cat_id, owner_id, weight_kg, logged_at, note, created_at, updated_at
         FROM weight_logs WHERE id = $1",
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
) -> Result<Vec<WeightLog>, AppError> {
    sqlx::query_as::<_, WeightLog>(
        "SELECT id, cat_id, owner_id, weight_kg, logged_at, note, created_at, updated_at
         FROM weight_logs
         WHERE cat_id = $1 AND owner_id = $2
         ORDER BY logged_at DESC
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
    req: &UpdateWeightLogRequest,
) -> Result<WeightLog, AppError> {
    let existing = find_by_id(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("weight log not found".to_string()))?;

    if existing.owner_id != *owner_id {
        return Err(AppError::Forbidden);
    }

    let weight_kg = req.weight_kg.unwrap_or(existing.weight_kg);
    let logged_at = req
        .logged_at
        .as_deref()
        .map(parse_datetime)
        .transpose()?
        .unwrap_or(existing.logged_at);
    let note: Option<String> = if req.note.is_some() {
        req.note.clone()
    } else {
        existing.note.clone()
    };
    let now = Utc::now();

    sqlx::query_as::<_, WeightLog>(
        "UPDATE weight_logs
         SET weight_kg = $1, logged_at = $2, note = $3, updated_at = $4
         WHERE id = $5 AND owner_id = $6
         RETURNING id, cat_id, owner_id, weight_kg, logged_at, note, created_at, updated_at",
    )
    .bind(weight_kg)
    .bind(logged_at)
    .bind(&note)
    .bind(now)
    .bind(id)
    .bind(owner_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn delete(db: &PgPool, id: &Uuid, owner_id: &Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM weight_logs WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("weight log not found".to_string()));
    }
    Ok(())
}

fn parse_datetime(s: &str) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|_| AppError::BadRequest("invalid datetime; expected ISO 8601".to_string()))
}
