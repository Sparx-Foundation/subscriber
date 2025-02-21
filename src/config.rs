use std::env;
use dotenvy::dotenv;

pub struct Config {
    pub(crate) smtp_name: String,
    pub(crate) smtp_pass: String,
    pub(crate) smtp_host: String,
    pub(crate) database_url: String,
}

impl Config {
    pub(crate) fn from_env() -> Self {
        dotenv().ok();

        let smtp_name = env::var("SMTP_NAME").expect("SMTP_NAME must be set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Config {
            smtp_name,
            smtp_pass,
            smtp_host,
            database_url,
        }
    }
}