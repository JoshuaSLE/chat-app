use fancy_regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static USERNAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

static PASSWORD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^A-Za-z0-9]).*$").unwrap());

fn validate_username(username: &str) -> Result<(), ValidationError> {
    match USERNAME_RE.is_match(username) {
        Ok(true) => Ok(()),
        _ => Err(ValidationError::new("format")),
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    match PASSWORD_RE.is_match(password) {
        Ok(true) => Ok(()),
        _ => Err(ValidationError::new("complexity")),
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
