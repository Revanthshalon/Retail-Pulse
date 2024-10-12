use serde::Deserialize;

use crate::errors::AppErrors;

#[derive(Debug, Deserialize)]
pub struct CreateTenantDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTenantDTO {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub contact: Option<String>,
}

// ================================================
// ================== VALIDATION ==================
// ================================================
impl CreateTenantDTO {
    pub fn validate(&self) -> Result<(), AppErrors> {
        if self.email.trim().is_empty() {
            return Err(AppErrors::ValidationError("Email is required".to_string()));
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

impl UpdateTenantDTO {
    pub fn validate(&self) -> Result<(), AppErrors> {
        if let Some(first_name) = &self.first_name {
            if first_name.trim().is_empty() {
                return Err(AppErrors::ValidationError(
                    "First Name is required".to_string(),
                ));
            }
        }
        if let Some(last_name) = &self.last_name {
            if last_name.trim().is_empty() {
                return Err(AppErrors::ValidationError(
                    "Last Name is required".to_string(),
                ));
            }
        }
        if let Some(country_code) = &self.country_code {
            if country_code.trim().is_empty() {
                return Err(AppErrors::ValidationError(
                    "Country Code is required".to_string(),
                ));
            }
            if country_code.len() != 2 {
                return Err(AppErrors::ValidationError(
                    "Country Code must be 2 characters".to_string(),
                ));
            }
            let mut valid = true;
            for c in country_code.chars() {
                if !c.is_ascii_digit() {
                    valid = false;
                    break;
                }
            }

            if !valid {
                return Err(AppErrors::ValidationError(
                    "Country Code must contain only digits".to_string(),
                ));
            }
        }
        if let Some(contact) = &self.contact {
            if contact.trim().is_empty() {
                return Err(AppErrors::ValidationError(
                    "Contact is required".to_string(),
                ));
            }
            if contact.len() < 10 {
                return Err(AppErrors::ValidationError(
                    "Contact must be at least 8 characters".to_string(),
                ));
            }

            let mut valid = true;

            for c in contact.chars() {
                if !c.is_ascii_digit() {
                    valid = false;
                    break;
                }
            }

            if !valid {
                return Err(AppErrors::ValidationError(
                    "Contact must contain only digits".to_string(),
                ));
            }
        }

        Ok(())
    }
}
