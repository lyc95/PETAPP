use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::warn;

use crate::{errors::AppError, state::AppState};

// ---------------------------------------------------------------------------
// JWKS cache
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct JwksCache(Arc<RwLock<Vec<CachedKey>>>);

struct CachedKey {
    kid: String,
    key: DecodingKey,
}

#[derive(Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

impl JwksCache {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(vec![])))
    }

    /// Fetches the JWKS from Cognito and populates the in-memory cache.
    /// Call once on startup from `main`.
    pub async fn load(&self, url: &str) -> anyhow::Result<()> {
        let set: JwkSet = reqwest::get(url).await?.json().await?;
        let mut guard = self.0.write().await;
        *guard = set
            .keys
            .into_iter()
            .filter_map(|jwk| {
                DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
                    .ok()
                    .map(|key| CachedKey { kid: jwk.kid, key })
            })
            .collect();
        tracing::info!("Loaded {} JWKS key(s)", guard.len());
        Ok(())
    }

    async fn find(&self, kid: &str) -> Option<DecodingKey> {
        self.0
            .read()
            .await
            .iter()
            .find(|k| k.kid == kid)
            .map(|k| k.key.clone())
    }
}

// ---------------------------------------------------------------------------
// Authenticated user — inserted into request extensions by the middleware
// ---------------------------------------------------------------------------

/// Authenticated caller. `sub` is the Cognito user ID used as `ownerId`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub sub: String,
}

// ---------------------------------------------------------------------------
// Middleware
// ---------------------------------------------------------------------------

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer(&request)?;

    let header = decode_header(token).map_err(|_| AppError::Unauthorized)?;
    let kid = header.kid.ok_or(AppError::Unauthorized)?;

    let decoding_key = state
        .jwks
        .find(&kid)
        .await
        .ok_or(AppError::Unauthorized)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[state.config.cognito_issuer()]);
    // Cognito access tokens use `client_id` instead of `aud` — skip aud check.
    validation.validate_aud = false;

    #[derive(Deserialize)]
    struct Claims {
        sub: String,
    }

    let data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
        warn!("JWT validation failed: {e}");
        AppError::Unauthorized
    })?;

    request
        .extensions_mut()
        .insert(AuthUser { sub: data.claims.sub });

    Ok(next.run(request).await)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_bearer(request: &Request) -> Result<&str, AppError> {
    request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)
}
