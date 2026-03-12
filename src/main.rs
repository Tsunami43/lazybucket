mod api;
mod config;
mod db;

use axum::{
    Router,
    extract::State,
    middleware,
    routing::{get, put},
};
use config::Config;
use sqlx::SqlitePool;

async fn health(State(state): State<AppState>) -> &'static str {
    println!("login: {}", state.config.login);
    "ok"
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    let pool = db::init_pool("sqlite://database.db?mode=rwc")
        .await
        .unwrap();
    let state = AppState { pool, config: cfg };

    let app = Router::new()
        .route("/health", get(health))
        .route("/buckets/:name", put(api::buckets::create_bucket))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api::middlewares::auth,
        ))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
