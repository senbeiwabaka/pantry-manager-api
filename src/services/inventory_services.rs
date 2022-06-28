use repository_db::Db;
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

use crate::models::{InventoryItem, Product};

use entity::inventory;
use entity::inventory::Entity as InventoryEntity;

pub fn add_inventory_item(product: &Product, count: u32) -> InventoryItem {
    InventoryItem {
        count: count,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: Some(product.clone()),
    }
}

pub async fn get_all_inventory(conn: Connection<'_, Db>) -> Vec<InventoryItem> {
    let db = conn.into_inner();
    let entities: Vec<inventory::Model> = InventoryEntity::find().all(db).await.ok().unwrap();

    dbg!(&entities);

    let mut results: Vec<InventoryItem> = Vec::new();

    for entity in entities {
        let result = InventoryItem {
            count: entity.count.unwrap() as u32,
            number_used_in_past_30_days: entity.number_used_in_past_thirty_days.unwrap() as u32,
            on_grocery_list: false,
            product: None,
        };

        results.push(result);
    }

    results
}
