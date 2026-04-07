use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user UUID
    iss: String,
    aud: String,
    exp: i64,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("password hash failed: {e}")))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid hash format: {e}")))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::BadRequest("incorrect password".to_string()))
}

/// Issue an HS256 JWT valid for 30 days.
pub fn issue_token(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("overflow computing token expiry")))?
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        iss: "catcare".to_string(),
        aud: "catcare".to_string(),
        exp,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("token sign failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let hash = hash_password("hunter2").unwrap();
        assert!(verify_password("hunter2", &hash).is_ok());
    }

    #[test]
    fn wrong_password_rejected() {
        let hash = hash_password("hunter2").unwrap();
        assert!(verify_password("wrong", &hash).is_err());
    }

    #[test]
    fn issue_and_decode_token() {
        let id = Uuid::new_v4();
        let token = issue_token(id, "test-secret").unwrap();
        assert!(!token.is_empty());
    }
}
