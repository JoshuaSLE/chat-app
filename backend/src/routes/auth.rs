use axum::{extract::State, http::StatusCode};

use crate::{AppState, error::AppError, extract::ValidatedJson, types::users::RegisterUser};

pub async fn register(
    State(_state): State<AppState>,
    ValidatedJson(_payload): ValidatedJson<RegisterUser>,
) -> Result<StatusCode, AppError> {
    tracing::info!("register endpoint");
    Ok(StatusCode::CREATED)
}
