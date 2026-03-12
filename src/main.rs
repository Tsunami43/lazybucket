mod api;
mod config;
mod db;

use axum::{
    Router,
    extract::State,
    middleware,
    routing::{get, put},
};
use config::{Config, PORT};
use sqlx::SqlitePool;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

async fn health(_state: State<AppState>) -> &'static str {
    "ok"
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
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
    let pool = db::init_pool("sqlite://database.db?mode=rwc")
        .await
        .unwrap();

    // AppState
    let state = AppState { pool, config: cfg };

    // App
    let app = Router::new()
        .route("/health", get(health))
        .route("/buckets/:name", put(api::buckets::create_bucket))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api::middlewares::auth,
        ))
        .with_state(state);

    // Server
    let addr = format!("0.0.0.0:{}", PORT);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    tracing::info!("DB connected");
    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
