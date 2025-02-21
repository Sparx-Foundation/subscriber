use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub(crate) smtp_name: String,
    pub(crate) smtp_pass: String,
    pub(crate) smtp_host: String,
    pub(crate) database_url: String,
    pub(crate) server_port: String,
    pub(crate) allowed_origins: Vec<String>,
}

impl Config {
    pub(crate) fn from_env() -> Self {
        dotenv().ok();

        let smtp_name = env::var("SMTP_NAME").expect("SMTP_NAME must be set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .expect("ALLOWED_ORIGINS must be set")
            .split(',')
            .map(|s| s.to_string())
            .collect();

        Config {
            smtp_name,
            smtp_pass,
            smtp_host,
            database_url,
            server_port,
            allowed_origins,
        }
    }
}
