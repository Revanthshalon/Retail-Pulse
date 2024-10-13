use tenant::{TenantRepository, TenantRepositoryTrait};

pub mod tenant;
mod auth;

pub struct RepositoryContainer {
    pub tenant: Box<dyn TenantRepositoryTrait>,
}

impl RepositoryContainer {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            tenant: Box::new(TenantRepository::new(pool.clone())),
        }
    }
}
