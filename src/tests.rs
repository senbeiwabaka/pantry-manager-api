use entity::inventory;
use entity::products;
use sea_orm::ActiveModelTrait;
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use sea_orm::Set;
use sea_orm::{DbBackend, DbConn, Schema};

use crate::models::InventoryItem;
use crate::models::Paged;
use crate::models::Product;
use crate::services::inventory_services;

#[async_test]
async fn add_inventory_item() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&db).await;

    let upc: String = "upc".to_string();

    let product_entity = get_product_entity(&upc);

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    product_entity.save(&db).await.unwrap();

    let product = get_product(&upc);

    let inventory_item = InventoryItem {
        count: 1,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: product.clone(),
    };

    // Act
    let result = inventory_services::add_inventory_item(&db, &product, 1).await;

    // Assert
    assert_eq!(inventory_item, result);
}

#[async_test]
async fn get_all_inventory_none() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&db).await;

    let expected: Paged<InventoryItem> = Paged::new();

    // Act
    let result = inventory_services::get_all_inventory(&db, None, None).await;

    // Assert
    assert_eq!(expected, result);
}

#[async_test]
async fn get_all_inventory() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&db).await;

    let upc: String = "upc".to_string();

    let product_entity = get_product_entity(&upc);

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    product_entity.save(&db).await.unwrap();

    let product = get_product(&upc);

    let inventory_item = InventoryItem {
        count: 1,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: product.clone(),
    };

    inventory_services::add_inventory_item(&db, &product, 1).await;

    let expected: Paged<InventoryItem> = Paged::<InventoryItem> {
        count: 1,
        data: vec![inventory_item],
    };

    // Act
    let result = inventory_services::get_all_inventory(&db, None, None).await;

    // Assert
    assert_eq!(expected.count, result.count);
    assert_eq!(expected.data, result.data);
}

#[async_test]
async fn get_inventory_by_upc() {
    // Arrange
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&db).await;

    let upc: String = "upc".to_string();

    get_product_entity(&upc).save(&db).await.unwrap_or_default();
    get_product_entity(&"upc 1".to_string())
        .save(&db)
        .await
        .unwrap_or_default();

    let product = get_product(&upc);

    let inventory_item = InventoryItem {
        count: 1,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: product.clone(),
    };

    inventory_services::add_inventory_item(&db, &product, 1).await;

    inventory_services::add_inventory_item(&db, &get_product(&"upc 1".to_string()), 3).await;

    // Act
    let result = inventory_services::get_inventory_by_upc(&db, &upc).await;

    // Assert
    assert_eq!(inventory_item, result);
}

async fn setup_schema(db: &DbConn) {
    let schema = Schema::new(DbBackend::Sqlite);
    let product_entity_table = schema.create_table_from_entity(products::Entity);
    let inventory_entity_table = schema.create_table_from_entity(inventory::Entity);

    // This is a test so if this fails that is fine. We have a bigger problem with the setup
    db.execute(db.get_database_backend().build(&product_entity_table))
        .await
        .unwrap();

    db.execute(db.get_database_backend().build(&inventory_entity_table))
        .await
        .unwrap();
}

fn get_product_entity(upc: &String) -> products::ActiveModel {
    products::ActiveModel {
        upc: Set(upc.to_owned()),
        ..Default::default()
    }
}

fn get_product(upc: &String) -> Product {
    Product {
        upc: upc.to_owned(),
        brand: None,
        category: None,
        image_url: None,
        label: "".to_string(),
    }
}
