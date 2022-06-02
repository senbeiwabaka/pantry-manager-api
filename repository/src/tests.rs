use entity::products;
use rocket::async_test;
use sea_orm::{entity::prelude::*, ConnectionTrait, Database, DbBackend, Schema, Set};

#[async_test]
async fn product_should_not_exist() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    // Act
    let result = crate::exists(&db, "upc".to_string()).await;

    // Assert
    assert!(!result);
}

#[async_test]
async fn product_should_exist() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&db).await;

    let entity = products::ActiveModel {
        upc: Set("upc".to_owned()),
        ..Default::default()
    };

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    entity.save(&db).await.unwrap();

    // Act
    let result = crate::exists(&db, "upc".to_string()).await;

    // Assert
    assert!(result);
}

async fn setup_schema(db: &DbConn) {
    let schema = Schema::new(DbBackend::Sqlite);
    let product_entity_table = schema.create_table_from_entity(products::Entity);

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    db.execute(db.get_database_backend().build(&product_entity_table))
        .await
        .unwrap();
}
