mod config;
use config::Config;

fn main() {
    let cfg = Config::from_env();

    println!("Hello, world! {}, {}", cfg.login, cfg.password);
}
