use tenant::TenantService;

use crate::repositories::RepositoryContainer;
use crate::services::auth::AuthService;

mod auth;
mod tenant;

pub struct ServiceContainer {
    pub tenant_service: TenantService,
    pub auth_service: AuthService,
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        let tenant_service = TenantService::new(repository_container.tenant.clone());
        let auth_service = AuthService::new(repository_container.tenant.clone());
        Self {
            tenant_service,
            auth_service,
        }
    }
}
