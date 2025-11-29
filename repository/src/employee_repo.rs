use crate::dao;
use crate::employee_errors::*;
use crate::postgres_db::PostgresDb;
use domain::dto;
use sqlx::PgPool;
use std::future::Future;
use uuid::Uuid;

pub trait EmployeeRepo: Send + Sync + Clone + 'static {
    fn new(pool: PgPool) -> Self;
    fn create(
        &self,
        ri: &dto::Employee,
    ) -> impl Future<Output = Result<dto::Employee, CreateEmployeeError>> + Send;
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<dto::Employee>, GetEmployeeError>> + Send;
    fn delete(&self, pet_id: Uuid) -> impl Future<Output = Result<(), DeleteEmployeeError>> + Send;
    fn update(
        &self,
        ri: &dto::Employee,
    ) -> impl Future<Output = Result<dto::Employee, UpdateEmployeeError>> + Send;
    fn get_all(&self)
    -> impl Future<Output = Result<Vec<dto::Employee>, GetEmployeesError>> + Send;
}

impl EmployeeRepo for PostgresDb {
    fn new(pool: PgPool) -> PostgresDb {
        PostgresDb { pool }
    }
    async fn create(&self, ri: &dto::Employee) -> Result<dto::Employee, CreateEmployeeError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| CreateEmployeeError::from_sqlx_with_email(err, &ri.email))?;
        let employee: dao::Employee = sqlx::query_as!(
            dao::Employee,
            "INSERT INTO employees(first_name, last_name, email, salary) VALUES ($1, $2, $3, $4) RETURNING *",
            ri.first_name,
            ri.last_name,
            ri.email,
            ri.salary,
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                CreateEmployeeError::from_sqlx_with_email(e, &ri.email)
            })?;
        tx.commit()
            .await
            .map_err(CreateEmployeeError::from_sqlx_commit_failed)?;
        Ok((&employee).into())
    }
    async fn update(&self, ri: &dto::Employee) -> Result<dto::Employee, UpdateEmployeeError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| UpdateEmployeeError::Unknown { source: e })?;
        let ret = sqlx::query_as!(
            dao::Employee,
            r#"UPDATE employees SET 
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
        .await
        .map_err(|e| UpdateEmployeeError::Unknown { source: e })?;
        tx.commit()
            .await
            .map_err(|e| UpdateEmployeeError::CommitFailed { source: e })?;
        Ok((&ret).into())
    }
    async fn delete(&self, id: Uuid) -> Result<(), DeleteEmployeeError> {
        sqlx::query_as!(dao::Employee, "DELETE FROM employees WHERE id=$1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DeleteEmployeeError::Unknown { source: e })?;
        Ok(())
    }
    async fn get(&self, id: Uuid) -> Result<Option<dto::Employee>, GetEmployeeError> {
        let ri = sqlx::query_as!(
            dao::Employee,
            "SELECT id, first_name, last_name, email, salary FROM employees where id=$1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| GetEmployeeError::Unknown { source: e })?;
        let ret: Option<dto::Employee> = ri.map(|x: dao::Employee| (&x).into());
        Ok(ret)
    }
    async fn get_all(&self) -> Result<Vec<dto::Employee>, GetEmployeesError> {
        let ris = sqlx::query_as!(
            dao::Employee,
            "SELECT id, first_name, last_name, email, salary FROM employees",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| GetEmployeesError::Unknown { source: e })?;
        Ok(ris.into_iter().map(|x| (&x).into()).collect())
    }
}
