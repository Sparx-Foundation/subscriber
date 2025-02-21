use crate::app_state::AppState;
use axum::{extract::State, Json};
use http::StatusCode;
use lettre::{message::header::ContentType, message::Mailbox, Address, Message, Transport};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, error};

use entity::email_subscriber;

#[derive(Deserialize, Debug)]
pub struct SubscribeRequests {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SubscribeRequests>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    debug!("Received subscribe request: {:?}", payload);

    let address = match payload.email.parse::<Address>() {
        Ok(add) => add,
        Err(err) => {
            error!("Invalid email address: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid email address".to_string(),
            ));
        }
    };

    let mail = match Message::builder()
        .from("BundeswareWTF <waitlist@bundesware.wtf>".parse().unwrap())
        .to(Mailbox::new(Some(payload.name.clone()), address))
        .subject("Welcome! - BundeswareWTF")
        .header(ContentType::TEXT_HTML)
        .body(String::from(include_str!("../../static/waitlist.html")))
    {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to build email message: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build email message".to_string(),
            ));
        }
    };

    let subscriber = email_subscriber::ActiveModel {
        name: Set(payload.name.to_string()),
        email: Set(payload.email.to_string()),
        ..Default::default()
    };

    match subscriber.insert(&state.db).await {
        Ok(_) => {
            debug!("Subscriber inserted successfully!");
        }
        Err(err) => {
            error!("Failed to insert subscriber: {}", err);

            return match email_subscriber::Entity::find()
                .filter(email_subscriber::Column::Email.eq(payload.email.to_string()))
                .one(&state.db)
                .await
            {
                Ok(sub) => {
                    debug!("Subscriber already exists: {:?}", sub);
                    Ok((
                        StatusCode::BAD_REQUEST,
                        "Subscriber already exists".to_string(),
                    ))
                }
                Err(err) => {
                    error!("Failed to find subscriber: {}", err);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Unknow Server Error".to_string(),
                    ))
                }
            };
        }
    };

    match state.smtp.send(&mail) {
        Ok(res) => {
            debug!("Email sent successfully: {:?}", res);
            Ok((StatusCode::OK, "Email sent successfully!".to_string()))
        }
        Err(err) => {
            error!("Failed to send email: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send email".to_string(),
            ))
        }
    }
}
