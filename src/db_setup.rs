use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use thiserror::Error;
use tracing::{error, info};

pub async fn setup_db(db_url: &str) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    info!("DB URL: {:?}", db_url);

    info!("Connecting to database...");
    let connection = match Database::connect(db_url).await {
        Ok(db) => {
            info!("Successfully connected to database");
            db
        }
        Err(e) => {
            error!("Database connection failed");
            error!("Troubleshooting tips:");
            error!("1. Check if PostgreSQL is running: 'pg_ctl status'");
            error!("2. Verify credentials in config/default.toml");
            error!("3. Ensure database exists: 'createdb {}'", db_url);
            return Err(e.into());
        }
    };

    Migrator::up(&connection, None).await?;

    Ok(connection)
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("Database '{0}' not found")]
    DatabaseNotFound(String),
    #[error("Unknown database error: {0}")]
    Unknown(String),
}

impl From<sea_orm::DbErr> for DatabaseError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::Conn(runtime_err) => match runtime_err.to_string() {
                e if e.contains("28P01") => DatabaseError::AuthenticationError(
                    "Invalid username or password. Please check your credentials.".to_string(),
                ),
                e if e.contains("3D000") => DatabaseError::DatabaseNotFound(
                    "Database does not exist. Make sure it's created.".to_string(),
                ),
                e => DatabaseError::ConnectionError(format!(
                    "Could not connect to database: {}. Check if PostgreSQL is running.",
                    e
                )),
            },
            _ => DatabaseError::Unknown(err.to_string()),
        }
    }
}
