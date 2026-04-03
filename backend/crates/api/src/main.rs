use axum::Router;
use lambda_http::{run, Error};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .merge(routes::health::router())
        .layer(cors);

    run(app).await
}
