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
    async fn check_if_tenant_exists(&self, id: i32) -> Result<bool, AppErrors> {
        let result = sqlx::query!("SELECT id FROM tenants WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn check_if_tenant_email_exists(&self, email: &str) -> Result<bool, AppErrors> {
        let result = sqlx::query!("SELECT email FROM tenants WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
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
        if self.check_if_tenant_email_exists(&payload.email).await? {
            return Err(AppErrors::Conflict("Email already exists".to_string()));
        }

        let result = sqlx::query_as!(
            Tenant,
            r#"
            INSERT INTO tenants (email, password)
            VALUES ($1, $2)
            RETURNING *
            "#,
            payload.email,
            payload.password,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_tenant_by_id(&self, id: i32) -> Result<Tenant, AppErrors> {
        let tenant_option = sqlx::query_as!(Tenant, r#"SELECT * FROM tenants WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        match tenant_option {
            Some(tenant) => Ok(tenant),
            None => Err(AppErrors::NotFound("Tenant not found".to_string())),
        }
    }

    async fn get_all_tenants(&self) -> Result<Vec<Tenant>, AppErrors> {
        let tenants = sqlx::query_as!(Tenant, r#"SELECT * FROM tenants"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(tenants)
    }

    async fn update_tenant(&self, id: i32, payload: UpdateTenantDTO) -> Result<Tenant, AppErrors> {
        if !self.check_if_tenant_exists(id).await? {
            return Err(AppErrors::NotFound("Tenant not found".to_string()));
        }

        let result = sqlx::query_as!(
            Tenant,
            r#"
            UPDATE tenants
            SET first_name = COALESCE($1, first_name),
            last_name = COALESCE($2, last_name),
            country_code = COALESCE($3, country_code),
            contact = COALESCE($4, contact),
            updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#,
            payload.first_name,
            payload.last_name,
            payload.country_code,
            payload.contact,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
    async fn delete_tenant(&self, id: i32) -> Result<bool, AppErrors> {
        if !self.check_if_tenant_exists(id).await? {
            return Err(AppErrors::NotFound("Tenant not found".to_string()));
        }

        let result = sqlx::query!("DELETE FROM tenants WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(result > 0)
    }
}
