use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, models::user::User};

// ---------------------------------------------------------------------------
// Internal DB row (includes password_hash not exposed in User)
// ---------------------------------------------------------------------------

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        Self {
            id: r.id,
            email: r.email,
            created_at: r.created_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(db: &PgPool, email: &str, password_hash: &str) -> Result<User, AppError> {
    sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)
         RETURNING id, email, password_hash, created_at",
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(db)
    .await
    .map(Into::into)
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint() == Some("users_email_key") => {
            AppError::BadRequest("email already registered".to_string())
        }
        other => AppError::Internal(other.into()),
    })
}

pub async fn find_by_email(db: &PgPool, email: &str) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, password_hash, created_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn find_by_id(db: &PgPool, id: &Uuid) -> Result<Option<User>, AppError> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, password_hash, created_at FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
    .map(|opt| opt.map(Into::into))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_row_converts_to_user() {
        let row = UserRow {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "$argon2...".to_string(),
            created_at: Utc::now(),
        };
        let user: User = row.into();
        assert_eq!(user.email, "test@example.com");
    }
}
