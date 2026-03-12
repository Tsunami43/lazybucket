mod api;
mod config;
mod db;
mod storage;

use axum::{
    Router, middleware,
    routing::{delete, get, patch, put},
};
use config::{Config, PORT, DATABASE_URL, STORAGE_PATH};
use sqlx::SqlitePool;
use std::sync::Arc;
use storage::local::LocalStorage;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
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
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let cfg = Config::from_env();

    let pool = db::init_pool(DATABASE_URL).await.unwrap();

    let state = AppState {
        pool,
        config: cfg,
        storage: Arc::new(LocalStorage::new(STORAGE_PATH)),
    };

    let protected = Router::new()
        .route("/buckets", get(api::handlers::buckets::list_buckets))
        .route("/buckets/:name", put(api::handlers::buckets::create_bucket))
        .route("/buckets/:name", delete(api::handlers::buckets::delete_bucket))
        .route("/buckets/:name", patch(api::handlers::buckets::rename_bucket))
        .route("/:bucket", get(api::handlers::objects::list_objects))
        .route("/:bucket/*key", put(api::handlers::objects::upload_object))
        .route("/:bucket/*key", delete(api::handlers::objects::delete_object))
        .route("/:bucket/*key", patch(api::handlers::objects::rename_object))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api::middlewares::auth,
        ));

    let spa = ServeDir::new("./frontend/dist")
        .fallback(ServeFile::new("./frontend/dist/index.html"));

    let app = Router::new()
        .route("/api/health", get(api::handlers::health::health))
        .route("/api/:bucket/*key", get(api::handlers::objects::download_object))
        .nest("/api", protected)
        .layer(CorsLayer::permissive())
        .with_state(state)
        .fallback_service(spa);

    let addr = format!("0.0.0.0:{}", PORT);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    tracing::info!("DB connected: {}", DATABASE_URL);
    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
