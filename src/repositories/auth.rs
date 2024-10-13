use crate::errors::AppErrors;
use crate::models::tenant::CreateTenantDTO;
use axum::async_trait;
use sqlx::PgPool;

pub struct AuthRepo {
    pool: PgPool,
}

impl AuthRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait AuthRepoTrait: Send + Sync {
    async fn register_user(&self, payload: CreateTenantDTO) -> Result<bool, AppErrors>;
    // async fn login_user(&self, )
}
