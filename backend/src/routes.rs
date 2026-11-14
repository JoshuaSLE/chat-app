use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

use crate::AppState;

mod auth;

// router function to define the routes for the application
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", auth_router())
}

// health route to check the status of the application
async fn health() -> StatusCode {
    StatusCode::OK
}

// auth router to handle authentication requests
fn auth_router() -> Router<AppState> {
    Router::new().route("/register", post(auth::register))
}
