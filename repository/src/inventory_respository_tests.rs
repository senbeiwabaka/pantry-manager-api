use entity::inventory;
use entity::products;
use rocket::async_test;
use sea_orm::{entity::prelude::*, ConnectionTrait, Database, DbBackend, Schema, Set};

use crate::repositories::inventory_repository;

#[async_test]
async fn inventory_item_should_not_exist() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    // Act
    let result = inventory_repository::exists(&db, "upc".to_string()).await;

    // Assert
    assert!(!result);
}

#[async_test]
async fn inventory_item_should_exist() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&db).await;

    let product_entity_active_model = products::ActiveModel {
        upc: Set("upc".to_owned()),
        ..Default::default()
    };

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    let xx: products::ActiveModel = product_entity_active_model.save(&db).await.unwrap();

    dbg!(&xx);

    let inventory_entity = inventory::ActiveModel {
        count: Set(Some(1)),
        number_used_in_past_thirty_days: Set(None),
        on_grocery_list: Set(Some(false)),
        product_id: xx.id,
        ..Default::default()
    };

    inventory_entity.save(&db).await.unwrap();

    // Act
    let result = inventory_repository::exists(&db, "upc".to_string()).await;

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

    let inventory_entity_table = schema.create_table_from_entity(inventory::Entity);

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    db.execute(db.get_database_backend().build(&inventory_entity_table))
        .await
        .unwrap();
}
