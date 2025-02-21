use crate::db_setup::setup_db;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::config::Config;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) smtp: SmtpTransport,
    pub(crate) db: DatabaseConnection,
    pub(crate) config: Config,
}

impl AppState {
    pub(crate) async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Loading configuration...");
        let config = Config::from_env();

        info!("SMTP Setup...");
        let creds = Credentials::new(config.smtp_name.clone(), config.smtp_pass.clone());

        let smtp = SmtpTransport::relay(&config.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        smtp.test_connection().unwrap();

        let db = setup_db(&config.database_url).await?;

        Ok(Self { smtp, db, config })
    }
}
