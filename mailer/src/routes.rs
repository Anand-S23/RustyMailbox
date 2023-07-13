use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{handlers::send_email, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/mailer", post(send_email))
        .with_state(app_state)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Rust Mailer backend api running!"
    }))
}
