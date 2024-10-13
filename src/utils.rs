use crate::errors::AppErrors;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

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
