use axum::async_trait;
use sqlx::PgPool;

use crate::{
    entities::tenant::Tenant,
    errors::AppErrors,
    models::tenant::{CreateTenantDTO, UpdateTenantDTO},
};

#[derive(Clone)]
pub struct TenantRepository {
    pool: PgPool,
}

impl TenantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait TenantRepositoryTrait: Send + Sync {
    async fn create_tenant(&self, payload: CreateTenantDTO) -> Result<Tenant, AppErrors>;
    async fn get_tenant_by_id(&self, id: i32) -> Result<Tenant, AppErrors>;
    async fn get_all_tenants(&self) -> Result<Vec<Tenant>, AppErrors>;
    async fn update_tenant(&self, id: i32, payload: UpdateTenantDTO) -> Result<Tenant, AppErrors>;
    async fn delete_tenant(&self, id: i32) -> Result<bool, AppErrors>;
}

#[async_trait]
impl TenantRepositoryTrait for TenantRepository {
    async fn create_tenant(&self, payload: CreateTenantDTO) -> Result<Tenant, AppErrors> {
        todo!()
    }

    async fn get_tenant_by_id(&self, id: i32) -> Result<Tenant, AppErrors> {
        todo!()
    }

    async fn get_all_tenants(&self) -> Result<Vec<Tenant>, AppErrors> {
        todo!()
    }

    async fn update_tenant(&self, id: i32, payload: UpdateTenantDTO) -> Result<Tenant, AppErrors> {
        todo!()
    }
    async fn delete_tenant(&self, id: i32) -> Result<bool, AppErrors> {
        todo!()
    }
}
