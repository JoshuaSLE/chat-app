use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static USERNAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

// Custom validator for username
fn validate_username(username: &str) -> Result<(), ValidationError> {
    if USERNAME_RE.is_match(username) {
        Ok(())
    } else {
        Err(ValidationError::new("format"))
    }
}

// Custom validator for password
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    if has_lower && has_upper && has_digit && has_special {
        Ok(())
    } else {
        Err(ValidationError::new("complexity"))
    }
}

#[derive(Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
        length(min = 3, max = 32, message = "Username must be 3-32 characters."),
        custom(
            function = "validate_username",
            message = "Username must contain letters, numbers, and underscores only."
        )
    )]
    pub username: String,

    #[validate(email(message = "Please enter a valid email address."))]
    pub email: String,

    #[validate(
        length(min = 8, max = 128, message = "Password must be 8-128 characters."),
        custom(
            function = "validate_password",
            message = "Password must contain at least one uppercase letter, one lowercase letter, one number, and one symbol."
        )
    )]
    pub password: String,

    #[validate(length(min = 1, max = 128, message = "Invitation code is required."))]
    pub invite_code: String,
}
