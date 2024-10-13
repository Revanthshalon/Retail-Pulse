use std::sync::Arc;
use tenant::{TenantRepository, TenantRepositoryTrait};

mod auth;
pub mod tenant;

pub struct RepositoryContainer {
    pub tenant: Arc<dyn TenantRepositoryTrait>,
}

impl RepositoryContainer {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            tenant: Arc::new(TenantRepository::new(pool.clone())),
        }
    }
}
