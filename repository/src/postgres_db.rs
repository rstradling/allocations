use sqlx::Error;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostgresDb {
    pub pool: PgPool,
}

impl PostgresDb {
    pub async fn create_db_pool(url: &str) -> Result<PgPool, Error> {
        PgPool::connect(url).await
    }
}
