mod config;
mod db;
// use config::Config;
use axum::Router;

#[tokio::main]
async fn main() {
    let _pool = db::init_pool("sqlite://database.db?mode=rwc")
        .await
        .unwrap();

    // let cfg = Config::from_env();
    let app = Router::new();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
