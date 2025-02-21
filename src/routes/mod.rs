
use crate::app_state::AppState;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

mod subscribe;

pub fn configure_routes(state: Arc<AppState>) -> Router {
    let origins = AllowOrigin::list(
        state
            .config
            .security
            .as_ref()
            .unwrap()
            .allow_origin
            .iter()
            .map(|origin| origin.parse().unwrap())
            .collect::<Vec<_>>(),
    );

    info!("Allowed Origins: {:?}", origins);
    info!("Configuring routes");

    Router::new()
        .route("/subscribe", post(subscribe::subscribe))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(180))) // abort request after 180sec
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([http::header::AUTHORIZATION, http::header::CONTENT_TYPE, http::header::ACCEPT, ])
                .allow_methods([http::Method::GET, http::Method::POST, http::Method::PUT]),
        )
        .layer(TraceLayer::new_for_http())
}
