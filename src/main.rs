use crate::routes::configure_routes;
use std::sync::Arc;
use tracing::{info, trace};

mod app_state;
mod config;
mod db_setup;
mod routes;
mod utility;

use crate::app_state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up!");

    human_panic::setup_panic!(human_panic::Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .authors(env!("CARGO_PKG_AUTHORS"))
        .homepage("https://github.com/Sparx-Foundation/subscribe")
        .support("- Open a support request on GitHub: https://github.com/Sparx-Foundation/subscribe/issues/new")
    );

    init_tracing!("RUST_LOG");

    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Server Backend");

    let state = Arc::new(AppState::new().await?);
    trace!("App State created!");

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &state.config.server_port)).await?;

    info!("Server Listening on:  {}", &state.config.server_port);

    let app = configure_routes(state);

    axum::serve(listener, app).await?;

    Ok(())
}
