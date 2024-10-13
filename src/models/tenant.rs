use serde::Deserialize;

use crate::errors::AppErrors;

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
            if contact.len() != 10 {
                return Err(AppErrors::ValidationError(
                    "Contact must be 10 digits".to_string(),
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
