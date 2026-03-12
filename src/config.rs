use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub login: String,
    pub password: String,
}

pub const PORT: u16 = 8000;
pub const DATABASE_URL: &str = "sqlite://database.db?mode=rwc";
pub const STORAGE_PATH: &str = "./storage";


impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let login = env::var("USER_LOGIN").expect("USER_LOGIN must be set");
        let password = env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");

        Config { login, password }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    #[test]
    fn test_config_loads() {
        unsafe {
            env::set_var("USER_LOGIN", "testuser");
            env::set_var("USER_PASSWORD", "testpass");
        }

        let cfg = super::Config::from_env();
        assert_eq!(cfg.login, "testuser");
        assert_eq!(cfg.password, "testpass");
    }
}
