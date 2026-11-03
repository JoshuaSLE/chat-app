use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

use crate::AppState;

mod auth;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", auth_router())
}

async fn health() -> StatusCode {
    StatusCode::OK
}

fn auth_router() -> Router<AppState> {
    Router::new().route("/register", post(auth::register))
}
