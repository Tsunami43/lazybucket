mod db;
// mod config;
// use config::Config;

#[tokio::main]
async fn main() {
    // let cfg = Config::from_env();

    let _pool = db::init_pool("sqlite://database.db?mode=rwc")
        .await
        .unwrap();
}
