use crate::entities::tenant::Tenant;
use crate::errors::AppErrors;
use crate::models::auth::{AuthLoginDTO, AuthRegisterDTO};
use crate::repositories::tenant::TenantRepositoryTrait;
use crate::utils::{generate_password_hash, verify_password_hash};
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
    pub async fn register(&self, payload: AuthRegisterDTO) -> Result<Tenant, AppErrors> {
        payload.validate()?;
        let password_hash = generate_password_hash(&payload.password)?;
        let user = self
            .repo
            .create_tenant(payload.email, password_hash)
            .await?;
        Ok(user)
    }

    pub async fn login(&self, payload: AuthLoginDTO) -> Result<Tenant, AppErrors> {
        // Implement the login logic here
        let user = self
            .repo
            .get_tenant_by_email(payload.email.as_ref())
            .await?;
        if !verify_password_hash(user.password.as_ref(), payload.password.as_ref())? {
            return Err(AppErrors::Unauthorized);
        }
        Ok(user)
    }
}
