use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    db::cats_repo,
    errors::AppError,
    models::{
        api_response::{ApiList, ApiResponse},
        cat::{Cat, CreateCatRequest, UpdateCatRequest},
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
        .route("/cats", get(list_cats).post(create_cat))
        .route(
            "/cats/:catId",
            get(get_cat).patch(update_cat).delete(delete_cat),
        )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list_cats(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ApiList<Cat>>, AppError> {
    let limit = pagination.limit.unwrap_or(DEFAULT_LIMIT).min(200);
    let offset = pagination.offset.unwrap_or(0).max(0);
    let cats = cats_repo::list_by_owner(&state.db, &auth.id, limit, offset).await?;
    Ok(Json(ApiList::new(cats)))
}

async fn create_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCatRequest>,
) -> Result<(StatusCode, Json<ApiResponse<Cat>>), AppError> {
    require_non_empty(&req.name, "name")?;
    require_non_empty(&req.breed, "breed")?;
    let birthdate = parse_date(&req.birthdate)?;
    let now = Utc::now();
    let cat = Cat {
        id: Uuid::new_v4(),
        owner_id: auth.id,
        name: req.name,
        breed: req.breed,
        birthdate,
        photo_key: req.photo_key,
        created_at: now,
        updated_at: now,
    };
    cats_repo::create(&state.db, &cat).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(cat))))
}

async fn get_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Cat>>, AppError> {
    let cat = cats_repo::find_by_id(&state.db, &cat_id)
        .await?
        .ok_or_else(|| AppError::NotFound("cat not found".to_string()))?;

    if cat.owner_id != auth.id {
        return Err(AppError::Forbidden);
    }
    Ok(Json(ApiResponse::ok(cat)))
}

async fn update_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
    Json(req): Json<UpdateCatRequest>,
) -> Result<Json<ApiResponse<Cat>>, AppError> {
    if let Some(name) = &req.name {
        require_non_empty(name, "name")?;
    }
    if let Some(breed) = &req.breed {
        require_non_empty(breed, "breed")?;
    }
    if let Some(bd) = &req.birthdate {
        parse_date(bd)?;
    }
    let cat = cats_repo::update(&state.db, &cat_id, &auth.id, &req).await?;
    Ok(Json(ApiResponse::ok(cat)))
}

async fn delete_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    cats_repo::delete(&state.db, &cat_id, &auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid birthdate; expected YYYY-MM-DD".to_string()))
}

fn require_non_empty(s: &str, field: &str) -> Result<(), AppError> {
    if s.trim().is_empty() {
        Err(AppError::BadRequest(format!("{field} must not be empty")))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_valid() {
        assert!(parse_date("2021-05-10").is_ok());
    }

    #[test]
    fn parse_date_slash_format_rejected() {
        assert!(parse_date("10/05/2021").is_err());
    }

    #[test]
    fn parse_date_free_text_rejected() {
        assert!(parse_date("not-a-date").is_err());
    }
}
