use crate::{
    entities::tenant::Tenant,
    errors::AppErrors,
    models::tenant::{CreateTenantDTO, UpdateTenantDTO},
    repositories::tenant::TenantRepositoryTrait,
};
use std::sync::Arc;

pub struct TenantService {
    repo: Arc<dyn TenantRepositoryTrait>,
}

impl TenantService {
    pub fn new(repo: Arc<dyn TenantRepositoryTrait>) -> Self {
        Self { repo }
    }
}

impl TenantService {
    pub async fn create_tenant(&self, payload: CreateTenantDTO) -> Result<Tenant, AppErrors> {
        self.repo.create_tenant(payload).await
    }
    pub async fn get_tenant_by_id(&self, id: i32) -> Result<Tenant, AppErrors> {
        self.repo.get_tenant_by_id(id).await
    }

    pub async fn get_all_tenants(&self) -> Result<Vec<Tenant>, AppErrors> {
        self.repo.get_all_tenants().await
    }

    pub async fn update_tenant(
        &self,
        id: i32,
        payload: UpdateTenantDTO,
    ) -> Result<Tenant, AppErrors> {
        self.repo.update_tenant(id, payload).await
    }

    pub async fn delete_tenant(&self, id: i32) -> Result<bool, AppErrors> {
        self.repo.delete_tenant(id).await
    }
}
