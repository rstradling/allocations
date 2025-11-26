use domain::dto;
use repository::dao;
use repository::postgres_db::PostgresDb;
use repository::roster_repo;
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
#[ignore = "integration"]
async fn test_create_pet(pool: PgPool) -> sqlx::Result<()> {
    let repo = PostgresDb::new_with_pool(pool);
    let roster_item = dao::RosterItem {
        id: uuid::Uuid::nil(),
        first_name: "foo".to_string(),
        last_name: "bar".to_string(),
        email: "foo@email.com".to_string(),
        salary: 1000.00,
    };
    let ret_roster_item =
        repository::postgres_db::PostgresDb::save_roster(&repo, &roster_item).await?;

    assert_ne!(ret_roster_item.id, uuid::Uuid::nil());
    assert_eq!(ret_roster_item.first_name, "foo");
    assert_eq!(ret_roster_item.last_name, "bar");
    assert_eq!(ret_roster_item.email, "foo@email.com");
    assert_eq!(ret_roster_item.salary, 1000.00);
    Ok(())
}

/*#[sqlx::test(migrations = "../migrations")]
#[ignore = "integration"]
async fn test_crud_pet(pool: PgPool) -> sqlx::Result<()> {
    let repo = PostgresDb::new_with_pool(pool);
    let pet = Pet {
        id: None,
        name: "foo".to_string(),
    };
    let ret_pet = repo.create_pet(&pet).await.unwrap();

    assert_eq!(ret_pet.name, "foo");
    assert!(ret_pet.id.is_some());

    let updated_pet = Pet {
        name: "bar".to_string(),
        id: ret_pet.id.clone(),
    };
    let ret2_pet = repo.update_pet(&updated_pet).await.unwrap();
    assert_eq!(ret2_pet.name, "bar");
    assert_eq!(ret2_pet.id, ret_pet.id.clone());

    let ret3_pet = repo
        .get_pet(ret_pet.id.clone().unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(ret3_pet.name, "bar");
    assert_eq!(ret3_pet.id, ret_pet.id);

    Ok(())
}*/
