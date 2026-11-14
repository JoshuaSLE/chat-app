use argon2::{Argon2, password_hash::PasswordHasher};
use axum::{extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{
    AppState, error::AppError, extract::ValidatedJson, invite::hash_code,
    types::users::RegisterUser,
};

/// Hashes the password for storage.
///
/// # Arguments
///
/// * `password` - The password to hash.
///
/// # Returns
///
/// * `Result<String, AppError>` - The hashed password as a string, or an error if the hashing fails.
fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|h| h.to_string())
        .map_err(|e| anyhow::anyhow!("password hashing failed: {e}").into())
}

/// Maps an error from a SQLx insert operation to an AppError.
///
/// # Arguments
///
/// * `e` - The SQLx error to map.
///
/// # Returns
///
/// * `AppError` - The mapped error.
fn map_user_insert_error(e: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db) = &e {
        if db.is_unique_violation() {
            match db.constraint() {
                Some("users_username_lower_idx") => {
                    return AppError::Conflict("Username already taken".into());
                }
                Some("users_email_lower_idx") => {
                    return AppError::Conflict("Email already registered".into());
                }
                _ => {}
            }
        }
    }
    e.into()
}

/// Registers a new user in the database.
///
/// # Arguments
///
/// * `State(state): State<AppState>` - The application state.
/// * `ValidatedJson(payload): ValidatedJson<RegisterUser>` - The validated JSON payload containing user registration details.
///
/// # Returns
///
/// * `Result
pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RegisterUser>,
) -> Result<StatusCode, AppError> {
    let username = payload.username.trim().to_string();
    let email = payload.email.trim().to_lowercase();
    let code_hash = hash_code(payload.invite_code.trim());

    let valid = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM invitations
            WHERE code_hash = $1 AND used_by IS NULL AND expires_at > now()
        ) as "valid!"
        "#,
        code_hash
    )
    .fetch_one(&state.db)
    .await?;

    if !valid {
        return Err(AppError::InvalidInvite);
    }

    let password = payload.password;
    let password_hash = tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(|e| anyhow::anyhow!("hash task failed: {e}"))??;

    let user_id = Uuid::now_v7();
    let mut tx = state.db.begin().await?;

    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password_hash)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        username,
        email,
        password_hash
    )
    .execute(&mut *tx)
    .await
    .map_err(map_user_insert_error)?;

    // Atomic claim: only one concurrent request can match this row.
    let claimed = sqlx::query!(
        r#"
        UPDATE invitations SET used_by = $1, used_at = now()
        WHERE code_hash = $2 AND used_by IS NULL AND expires_at > now()
        "#,
        user_id,
        code_hash
    )
    .execute(&mut *tx)
    .await?;

    if claimed.rows_affected() == 0 {
        return Err(AppError::InvalidInvite); // tx dropped -> rolled back
    }

    tx.commit().await?;
    Ok(StatusCode::CREATED)
}
