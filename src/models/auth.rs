use crate::errors::AppErrors;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthRegisterDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthLoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponseDTO {
    pub token: String,
}

// ================================================
// ================== VALIDATION ==================
// ================================================
impl AuthRegisterDTO {
    pub fn validate(&self) -> Result<(), AppErrors> {
        if self.email.trim().is_empty() {
            return Err(AppErrors::ValidationError("Email is required".to_string()));
        }
        if !self.email.trim().contains("@")
            || !self.email.trim().contains(".")
            || !self.email.split("@").count() == 2
        {
            return Err(AppErrors::ValidationError("Email is invalid".to_string()));
        }

        if self.password.trim().is_empty() {
            return Err(AppErrors::ValidationError(
                "Password is required".to_string(),
            ));
        }
        if self.password.len() < 8 {
            return Err(AppErrors::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        for c in self.password.chars() {
            if c.is_uppercase() {
                has_upper = true;
            }
            if c.is_lowercase() {
                has_lower = true;
            }
            if c.is_ascii_digit() {
                has_digit = true;
            }
        }

        if !has_upper || !has_lower || !has_digit {
            return Err(AppErrors::ValidationError(
                "Password must contain at least one uppercase, one lowercase, and one digit"
                    .to_string(),
            ));
        }
        Ok(())
    }
}
