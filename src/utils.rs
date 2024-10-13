use crate::config::AppConfig;
use crate::entities::tenant::Tenant;
use crate::errors::AppErrors;
use crate::models::auth::Claims;
use crate::AppState;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn generate_password_hash(password: &str) -> Result<String, AppErrors> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppErrors::HashingError(e.to_string()))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password_hash(hash: &str, password: &str) -> Result<bool, AppErrors> {
    let password_hash =
        argon2::PasswordHash::new(hash).map_err(|e| AppErrors::HashingError(e.to_string()))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok())
}

pub fn generate_token(app_config: &AppConfig, user: &Tenant) -> Result<String, AppErrors> {
    let expiration_option =
        Utc::now().checked_add_signed(Duration::seconds(app_config.jwt_expiry()));

    let expiration = match expiration_option {
        Some(exp) => exp.timestamp(),
        None => {
            return Err(AppErrors::InternalServerError(
                "Failed to generate token".to_string(),
            ));
        }
    };

    let claims = Claims {
        sub: user.id,
        exp: expiration,
        iat: Utc::now().timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_config.jwt_secret().as_bytes()),
    )
    .map_err(|e| AppErrors::InternalServerError(e.to_string()))?;

    Ok(token)
}

pub fn verify_token(app_state: &AppState, token: &str) -> Result<Claims, AppErrors> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(app_state.config.jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppErrors::Unauthorized)?;

    Ok(token_data.claims)
}
