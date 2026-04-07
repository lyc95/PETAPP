use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::cat::{Cat, UpdateCatRequest},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(db: &PgPool, cat: &Cat) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO cats (id, owner_id, name, breed, birthdate, photo_key, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(cat.id)
    .bind(cat.owner_id)
    .bind(&cat.name)
    .bind(&cat.breed)
    .bind(cat.birthdate)
    .bind(&cat.photo_key)
    .bind(cat.created_at)
    .bind(cat.updated_at)
    .execute(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: &Uuid) -> Result<Option<Cat>, AppError> {
    sqlx::query_as::<_, Cat>(
        "SELECT id, owner_id, name, breed, birthdate, photo_key, created_at, updated_at
         FROM cats WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn list_by_owner(
    db: &PgPool,
    owner_id: &Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Cat>, AppError> {
    sqlx::query_as::<_, Cat>(
        "SELECT id, owner_id, name, breed, birthdate, photo_key, created_at, updated_at
         FROM cats
         WHERE owner_id = $1
         ORDER BY created_at DESC
         LIMIT $2 OFFSET $3",
    )
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
    req: &UpdateCatRequest,
) -> Result<Cat, AppError> {
    let existing = find_by_id(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("cat not found".to_string()))?;

    if existing.owner_id != *owner_id {
        return Err(AppError::Forbidden);
    }

    let name = req.name.as_deref().unwrap_or(&existing.name).to_string();
    let breed = req.breed.as_deref().unwrap_or(&existing.breed).to_string();
    let birthdate = req
        .birthdate
        .as_deref()
        .map(parse_date)
        .transpose()?
        .unwrap_or(existing.birthdate);
    let photo_key: Option<String> = if req.photo_key.is_some() {
        req.photo_key.clone()
    } else {
        existing.photo_key.clone()
    };
    let now = Utc::now();

    sqlx::query_as::<_, Cat>(
        "UPDATE cats
         SET name = $1, breed = $2, birthdate = $3, photo_key = $4, updated_at = $5
         WHERE id = $6 AND owner_id = $7
         RETURNING id, owner_id, name, breed, birthdate, photo_key, created_at, updated_at",
    )
    .bind(&name)
    .bind(&breed)
    .bind(birthdate)
    .bind(&photo_key)
    .bind(now)
    .bind(id)
    .bind(owner_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn delete(db: &PgPool, id: &Uuid, owner_id: &Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM cats WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("cat not found".to_string()));
    }
    Ok(())
}

fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid birthdate; expected YYYY-MM-DD".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_valid() {
        assert!(parse_date("2022-03-15").is_ok());
    }

    #[test]
    fn parse_date_invalid() {
        assert!(parse_date("not-a-date").is_err());
    }
}
