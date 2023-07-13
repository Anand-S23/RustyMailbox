mod handlers;
mod middleware;
mod routes;

use axum::http::Method;
use dotenv::dotenv;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use routes::create_router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    mailer: SmtpTransport,
    to_email: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not provided");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not provided");
    let to_email = std::env::var("TO_EMAIL").expect("TO_EMAIL not provided");

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = create_router(Arc::new(AppState {
        mailer: mailer.clone(),
        to_email: to_email.to_owned(),
    }))
    .layer(cors);

    println!("🚀 Rust mailer server started successfully");
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
