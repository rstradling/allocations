use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostgresDb {
    pub pool: PgPool,
}
