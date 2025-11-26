use crate::converters;
use crate::dao;
use crate::postgres_db::PostgresDb;
use domain::dto;
use sqlx::PgPool;
use uuid::Uuid;

impl PostgresDb {
    pub async fn new(url: &str) -> Result<PostgresDb, sqlx::Error> {
        let pool = PgPool::connect(url).await?;
        Ok(PostgresDb { pool })
    }
    pub fn new_with_pool(pool: PgPool) -> PostgresDb {
        PostgresDb { pool }
    }
    pub async fn save_roster(&self, ri: &dao::RosterItem) -> Result<dto::RosterItem, sqlx::Error> {
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
    pub async fn update_roster(
        &self,
        ri: &dao::RosterItem,
    ) -> Result<dto::RosterItem, sqlx::Error> {
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
    pub async fn delete_roster(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query_as!(dao::RosterItem, "DELETE FROM roster WHERE id=$1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn get_roster(&self, id: Uuid) -> Result<Option<dto::RosterItem>, sqlx::Error> {
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
    pub async fn get_whole_roster(&self) -> Result<Vec<dto::RosterItem>, sqlx::Error> {
        let ris = sqlx::query_as!(
            dao::RosterItem,
            "SELECT id, first_name, last_name, email, salary FROM roster",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ris.into_iter().map(|x| (&x).into()).collect())
    }
}
