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
