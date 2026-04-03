use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::{NaiveDate, Utc};
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
) -> Result<Json<ApiList<Cat>>, AppError> {
    let cats =
        cats_repo::list_by_owner(&state.dynamo, &state.config.cats_table, &auth.sub).await?;
    Ok(Json(ApiList::ok(cats)))
}

async fn create_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCatRequest>,
) -> Result<(StatusCode, Json<ApiResponse<Cat>>), AppError> {
    let birthdate = parse_date(&req.birthdate)?;
    let now = Utc::now();
    let cat = Cat {
        id: Uuid::new_v4(),
        owner_id: auth.sub,
        name: req.name,
        breed: req.breed,
        birthdate,
        photo_key: req.photo_key,
        created_at: now,
        updated_at: now,
    };
    cats_repo::create(&state.dynamo, &state.config.cats_table, &cat).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(cat))))
}

async fn get_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Cat>>, AppError> {
    let cat = cats_repo::find_by_id(&state.dynamo, &state.config.cats_table, &cat_id)
        .await?
        .ok_or_else(|| AppError::NotFound("cat not found".to_string()))?;

    if cat.owner_id != auth.sub {
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
    // Validate birthdate if provided.
    if let Some(bd) = &req.birthdate {
        parse_date(bd)?;
    }
    let cat =
        cats_repo::update(&state.dynamo, &state.config.cats_table, &cat_id, &auth.sub, &req)
            .await?;
    Ok(Json(ApiResponse::ok(cat)))
}

async fn delete_cat(
    Extension(auth): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Path(cat_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    cats_repo::delete(&state.dynamo, &state.config.cats_table, &cat_id, &auth.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid birthdate; expected YYYY-MM-DD".to_string()))
}
