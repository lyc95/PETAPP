use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod state;

use auth::auth_middleware;
use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let config = Config::from_env().expect("failed to load config from env");
    let port = config.port;
    let cfg = Arc::new(config);

    let db = sqlx::PgPool::connect(&cfg.database_url)
        .await
        .expect("failed to connect to PostgreSQL");

    sqlx::migrate!("../../migrations")
        .run(&db)
        .await
        .expect("failed to run database migrations");

    tracing::info!("Migrations applied successfully");

    // Ensure the upload directory exists
    tokio::fs::create_dir_all(&cfg.upload_dir)
        .await
        .expect("failed to create upload directory");

    let state = Arc::new(AppState { config: cfg, db });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    // Public routes — no auth required.
    let public = Router::new()
        .route("/health", get(routes::health::handler))
        .merge(routes::auth::router());

    // Protected routes — require a valid JWT.
    let protected = routes::cats::router()
        .merge(routes::uploads::router())
        .merge(routes::medicine_reminders::router())
        .merge(routes::weight_logs::router())
        .merge(routes::health_records::router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = public.merge(protected).layer(cors).with_state(state);

    let addr = format!("0.0.0.0:{port}");
    tracing::info!("Listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
