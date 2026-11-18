use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    extract::ValidatedJson,
    invite::hash_code,
    types::users::{LoginUser, RegisterUser, UserResponse},
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
/// * `State(state)` - The application state.
/// * `ValidatedJson(payload)` - The validated JSON payload containing user registration details.
///
/// # Returns
///
/// * `Result<StatusCode, AppError>` - The result of the registration process.
///
/// # Errors
///
/// * `AppError::Conflict` - If the username or email already exists.
/// * `AppError::InvalidInvite` - If the invite code is invalid.
/// * `AppError::InternalError` - If there is an internal error during the registration process.
pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RegisterUser>,
) -> Result<StatusCode, AppError> {
    let username = payload.username.trim().to_string();
    let email = payload.email.trim();
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

/// Login user with provided credentials.
///
/// # Arguments
/// * `State(state): State<AppState>` - Application state.
/// * `jar: CookieJar` - Cookie jar for session management.
/// * `ValidatedJson(payload): ValidatedJson<LoginUser>` - Login user payload.
///
/// # Returns
/// * `Result<(CookieJar, Json<UserResponse>), StatusCode>` - Cookie jar and user response.
///
/// # Errors
/// * `AppError::InvalidCredentials` - Invalid credentials.
/// * `AppError::InvalidUser` - Invalid user.
/// * `AppError::InternalServerError` - Internal server error.
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<LoginUser>,
) -> Result<(CookieJar, Json<UserResponse>), StatusCode> {
    let username = payload.username.trim();
    let password = payload.password;

    struct UserRow {
        id: Uuid,
        username: String,
        password_hash: String,
    }

    let row = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, username, password_hash FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let hash = row
        .as_ref()
        .map(|r| r.password_hash.clone())
        .unwrap_or_else(|| state.dummy_hash.clone());

    let valid = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .map(|h| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &h)
                    .is_ok()
            })
            .unwrap_or(false)
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match (row, valid) {
        (Some(u), true) => u,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut raw = [0u8; 32];
    rand::rng().fill_bytes(&mut raw);
    let token = URL_SAFE_NO_PAD.encode(raw);
    let token_hash = Sha256::digest(token.as_bytes()).to_vec();

    sqlx::query!(
        "INSERT INTO sessions (token_hash, user_id, expires_at)
             VALUES ($1, $2, now() + interval '7 days')",
        token_hash,
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = Cookie::build(("session", token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7));

    Ok((
        jar.add(cookie),
        Json(UserResponse {
            id: user.id,
            username: user.username,
        }),
    ))
}
