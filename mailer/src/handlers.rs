use crate::AppState;
use axum::extract::State;
use axum::Json;
use lettre::{message::header::ContentType, Message, Transport};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct MailerInput {
    pub name: String,
    pub email: String,
    pub message: String,
}

pub async fn send_email(State(data): State<Arc<AppState>>, Json(input): Json<MailerInput>) {
    // TODO: Validate input

    let to_from_address = format!("<{}>", data.to_email);

    let body = format!(
        "Email from {} <{}>,\n\n{}",
        input.name, input.email, input.message
    );

    let email = Message::builder()
        .from(to_from_address.parse().unwrap())
        .to(to_from_address.parse().unwrap())
        .subject("Email From Mailer")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    match data.mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {e:?}"),
    }
}
