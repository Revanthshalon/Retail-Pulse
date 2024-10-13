use crate::errors::AppErrors;
use crate::models::auth::{AuthLoginDTO, AuthRegisterDTO, AuthResponseDTO};
use crate::repositories::tenant::TenantRepositoryTrait;
use std::sync::Arc;

pub struct AuthService {
    repo: Arc<dyn TenantRepositoryTrait>,
}

impl AuthService {
    pub fn new(repo: Arc<dyn TenantRepositoryTrait>) -> Self {
        Self { repo }
    }
}

impl AuthService {
    pub async fn register(&self, payload: AuthRegisterDTO) -> Result<AuthResponseDTO, AppErrors> {
        // Implement the register logic here
        todo!()
    }

    pub async fn login(&self, payload: AuthLoginDTO) -> Result<AuthResponseDTO, AppErrors> {
        // Implement the login logic here
        todo!()
    }
}
