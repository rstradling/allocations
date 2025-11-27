use crate::dao;
use crate::postgres_db::PostgresDb;
use domain::dto;
use sqlx::PgPool;
use std::future::Future;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RosterItemError {
    #[error("Failed to create roster_item with id {id}: {source}")]
    CreateFailed { id: Uuid, source: sqlx::Error },

    #[error("Duplicate pet with the same id: {id}: {source}")]
    Duplicate { id: Uuid, source: sqlx::Error },

    #[error("Failed to commit transactions: {source}")]
    CommitFailed { source: sqlx::Error },

    #[error("Unknown error for roster_item with id {id}: {source}")]
    Unknown { id: Uuid, source: sqlx::Error },

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl From<RosterItemError> for sqlx::Error {
    fn from(error: RosterItemError) -> Self {
        match error {
            RosterItemError::CreateFailed { id: _, source } => source,
            RosterItemError::Duplicate { id: _, source } => source,
            RosterItemError::CommitFailed { source } => source,
            RosterItemError::Unknown { id: _, source } => source,
            RosterItemError::Sqlx(source) => source,
        }
    }
}

pub trait RosterRepo: Send + Sync + Clone + 'static {
    fn new(pool: PgPool) -> Self;
    fn create(
        &self,
        ri: &dto::RosterItem,
    ) -> impl Future<Output = Result<dto::RosterItem, RosterItemError>> + Send;
    fn get(
        &self,
        pet_id: Uuid,
    ) -> impl Future<Output = Result<Option<dto::RosterItem>, RosterItemError>> + Send;
    fn delete(&self, pet_id: Uuid) -> impl Future<Output = Result<(), RosterItemError>> + Send;
    fn update(
        &self,
        ri: &dto::RosterItem,
    ) -> impl Future<Output = Result<dto::RosterItem, RosterItemError>> + Send;
    fn get_all(&self)
    -> impl Future<Output = Result<Vec<dto::RosterItem>, RosterItemError>> + Send;
}

impl RosterRepo for PostgresDb {
    fn new(pool: PgPool) -> PostgresDb {
        PostgresDb { pool }
    }
    async fn create(&self, ri: &dto::RosterItem) -> Result<dto::RosterItem, RosterItemError> {
        let mut tx = self.pool.begin().await?;
        let roster_item: dao::RosterItem = sqlx::query_as!(
            dao::RosterItem,
            "INSERT INTO roster(first_name, last_name, email, salary) VALUES ($1, $2, $3, $4) RETURNING *",
            ri.first_name,
            ri.last_name,
            ri.email,
            ri.salary,
            )
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok((&roster_item).into())
    }
    async fn update(&self, ri: &dto::RosterItem) -> Result<dto::RosterItem, RosterItemError> {
        let mut tx = self.pool.begin().await?;
        let ret = sqlx::query_as!(
            dao::RosterItem,
            r#"UPDATE roster SET 
                first_name = $1,
                last_name = $2,
                email = $3,
                salary = $4 
               WHERE id = $5 RETURNING *"#,
            ri.first_name,
            ri.last_name,
            ri.email,
            ri.salary,
            ri.id
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok((&ret).into())
    }
    async fn delete(&self, id: Uuid) -> Result<(), RosterItemError> {
        sqlx::query_as!(dao::RosterItem, "DELETE FROM roster WHERE id=$1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn get(&self, id: Uuid) -> Result<Option<dto::RosterItem>, RosterItemError> {
        let ri = sqlx::query_as!(
            dao::RosterItem,
            "SELECT id, first_name, last_name, email, salary FROM roster where id=$1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        let ret: Option<dto::RosterItem> = ri.map(|x: dao::RosterItem| (&x).into());
        Ok(ret)
    }
    async fn get_all(&self) -> Result<Vec<dto::RosterItem>, RosterItemError> {
        let ris = sqlx::query_as!(
            dao::RosterItem,
            "SELECT id, first_name, last_name, email, salary FROM roster",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ris.into_iter().map(|x| (&x).into()).collect())
    }
}
