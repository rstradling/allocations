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
    /*async fn update(&self, pet: &dao::Pet) -> Result<Pet, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let ret_pet = sqlx::query_as!(
            dao::Pet,
            "UPDATE pets SET name = $1 WHERE id = $2 RETURNING *",
            pet.name,
            pet.id
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(ret_pet.into())
    }
    async fn get(&self, id: Uuid) -> Result<Option<Pet>, sqlx::Error> {
        let pet = sqlx::query_as!(
            pet_share_domain::dao::Pet,
            "SELECT id, name FROM pets WHERE id=$1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        let pet_ret: Option<Pet> = pet.map(|x| x.into());
        Ok(pet_ret)
    }*/
}
