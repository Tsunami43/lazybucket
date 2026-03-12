mod api;
mod config;
mod db;
mod storage;

use axum::{
    Router, middleware,
    routing::{delete, get, patch, put},
};
use config::{Config, DATABASE_URL, PORT, STORAGE_PATH};
use sqlx::SqlitePool;
use std::sync::Arc;
use storage::local::LocalStorage;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
    pub storage: Arc<LocalStorage>,
}

#[tokio::main]
async fn main() {
    // Logger
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    // Config
    let cfg = Config::from_env();

    // DB pool
    let pool = db::init_pool(DATABASE_URL).await.unwrap();

    // AppState
    let state = AppState {
        pool,
        config: cfg,
        storage: Arc::new(LocalStorage::new(STORAGE_PATH)),
    };

    // App
    let protected = Router::new()
        .route("/buckets", get(api::handlers::buckets::list_buckets))
        .route("/buckets/:name", put(api::handlers::buckets::create_bucket))
        .route("/buckets/:name", delete(api::handlers::buckets::delete_bucket))
        .route("/buckets/:name", patch(api::handlers::buckets::rename_bucket))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api::middlewares::auth,
        ));

    let app = Router::new()
        .route("/health", get(api::handlers::health::health))
        .merge(protected)
        .with_state(state);

    // Server
    let addr = format!("0.0.0.0:{}", PORT);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    tracing::info!("DB connected: {}", DATABASE_URL);
    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
