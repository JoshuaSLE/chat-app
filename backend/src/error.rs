use std::collections::HashMap;

use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation failed")]
    Validation(#[from] ValidationErrors),

    #[error("invalid request body")]
    BadJson(#[from] JsonRejection),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorBody {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<HashMap<String, Vec<String>>>,
}

fn field_messages(errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
    errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let msgs = errs
                .iter()
                .map(|e| match &e.message {
                    Some(m) => m.to_string(),
                    None => format!("Invalid value ({})", e.code),
                })
                .collect();
            (field.to_string(), msgs)
        })
        .collect()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, fields) = match &self {
            AppError::Validation(errs) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_failed",
                "One or more fields are invalid".to_string(),
                Some(field_messages(errs)),
            ),
            AppError::BadJson(rej) => (
                rej.status(),
                "invalid_body",
                "Request body is missing or malformed".to_string(),
                None,
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication required".to_string(),
                None,
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "You do not have permission to do that".to_string(),
                None,
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "Resource not found".to_string(),
                None,
            ),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone(), None),
            AppError::Database(_) | AppError::Internal(_) => {
                tracing::error!(error = ?self, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "Something went wrong".to_string(),
                    None,
                )
            }
        };

        (
            status,
            Json(ErrorBody {
                error: ErrorDetail {
                    code,
                    message,
                    fields,
                },
            }),
        )
            .into_response()
    }
}
