use bigdecimal::BigDecimal;
use domain::dto;
use repository::dao;
use repository::postgres_db::PostgresDb;
use repository::roster_repo::RosterRepo;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = "../migrations")]
#[ignore = "integration"]
async fn test_create_roster(pool: PgPool) -> sqlx::Result<()> {
    let repo: PostgresDb = RosterRepo::new(pool);
    let roster_item = dao::RosterItem {
        id: uuid::Uuid::nil(),
        first_name: "foo".to_string(),
        last_name: "bar".to_string(),
        email: "foo@email.com".to_string(),
        salary: "1000.00".parse::<BigDecimal>().unwrap(),
    };
    let ret_roster_item = repo.save(&roster_item).await?;

    assert_ne!(ret_roster_item.id, uuid::Uuid::nil());
    assert_eq!(ret_roster_item.first_name, "foo");
    assert_eq!(ret_roster_item.last_name, "bar");
    assert_eq!(ret_roster_item.email, "foo@email.com");
    assert_eq!(
        ret_roster_item.salary,
        "1000.00".parse::<BigDecimal>().unwrap()
    );
    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
#[ignore = "integration"]
async fn test_get_whole_roster(pool: PgPool) -> sqlx::Result<()> {
    let repo: PostgresDb = RosterRepo::new(pool);
    let ri = dao::RosterItem {
        id: Uuid::nil(),
        first_name: "run".to_string(),
        last_name: "away".to_string(),
        email: "run@away.com".to_string(),
        salary: "32_000.00".parse::<BigDecimal>().unwrap(),
    };
    let ret_ri = repo.save(&ri).await.unwrap();
    let ri2 = dao::RosterItem {
        id: Uuid::nil(),
        first_name: "foo".to_string(),
        last_name: "bar".to_string(),
        email: "this@away.com".to_string(),
        salary: "30_000.00".parse::<BigDecimal>().unwrap(),
    };
    let ret_ri2 = repo.save(&ri2).await.unwrap();

    let roster_items: Vec<dto::RosterItem> = repo.get_all().await?;
    assert_eq!(roster_items.len(), 2);
    assert_eq!(roster_items[0], ret_ri);
    assert_eq!(roster_items[1], ret_ri2);
    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
#[ignore = "integration"]
async fn test_crud_roster(pool: PgPool) -> sqlx::Result<()> {
    let repo: PostgresDb = RosterRepo::new(pool);
    let ri = dao::RosterItem {
        id: Uuid::nil(),
        first_name: "run".to_string(),
        last_name: "away".to_string(),
        email: "run@away.com".to_string(),
        salary: "32_000.00".parse::<BigDecimal>().unwrap(),
    };
    let ret_ri = repo.save(&ri).await.unwrap();

    assert_eq!(ret_ri.first_name, "run");
    assert_eq!(ret_ri.last_name, "away");
    assert_eq!(ret_ri.email, "run@away.com");
    assert_eq!(ret_ri.salary, "32_000.00".parse::<BigDecimal>().unwrap());
    assert_ne!(ret_ri.id, uuid::Uuid::nil());

    let updated_pet = dao::RosterItem {
        first_name: "bar".to_string(),
        last_name: "another".to_string(),
        email: "bar@another.com".to_string(),
        salary: "45_000.00".parse::<BigDecimal>().unwrap(),
        id: ret_ri.id,
    };
    let ret2_ri = repo.update(&updated_pet).await.unwrap();
    assert_eq!(ret2_ri.first_name, "bar");
    assert_eq!(ret2_ri.last_name, "another");
    assert_eq!(ret2_ri.email, "bar@another.com");
    assert_eq!(ret2_ri.salary, "45_000.00".parse::<BigDecimal>().unwrap());
    assert_eq!(ret2_ri.id, ret_ri.id.clone());

    let ret3_ri: dto::RosterItem = repo.get(ret_ri.id).await.unwrap().unwrap();
    assert_eq!(ret3_ri.first_name, "bar");
    assert_eq!(ret3_ri.last_name, "another");
    assert_eq!(ret3_ri.email, "bar@another.com");
    assert_eq!(ret3_ri.salary, "45_000.00".parse::<BigDecimal>().unwrap());
    assert_eq!(ret3_ri.id, ret_ri.id.clone());

    repo.delete(ret_ri.id).await?;
    let ret3_ri = repo.get(ret_ri.id).await.unwrap();
    assert!(ret3_ri.is_none());

    Ok(())
}
