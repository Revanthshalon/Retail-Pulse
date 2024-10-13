use tenant::TenantService;

use crate::repositories::RepositoryContainer;

mod tenant;
mod auth;

pub struct ServiceContainer {
    pub tenant_service: TenantService,
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        let tenant_service = TenantService::new(repository_container.tenant);
        Self { tenant_service }
    }
}
