use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::models::{InventoryItem, Product};

use entity::inventory;
use entity::inventory::Entity as InventoryEntity;
use entity::products::Entity as ProductEntity;

pub async fn add_inventory_item(
    db: &DatabaseConnection,
    product: &Product,
    count: u32,
) -> InventoryItem {
    let entity = inventory::ActiveModel {
        count: Set(Some(count as i32)),
        number_used_in_past_thirty_days: Set(Some(0)),
        on_grocery_list: Set(Some(false)),
        product_id: Set(1),
        ..Default::default()
    };

    entity.save(db).await.unwrap();

    InventoryItem {
        count: count,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: Some(product.clone()),
    }
}

pub async fn get_all_inventory(db: &DatabaseConnection) -> Vec<InventoryItem> {
    let entities = InventoryEntity::find()
        .find_also_related(ProductEntity)
        .all(db)
        .await
        .ok()
        .unwrap();

    dbg!(&entities);

    let mut results: Vec<InventoryItem> = Vec::new();

    for entity in entities {
        dbg!(&entity);

        let product_entity = entity.1.unwrap();
        let result = InventoryItem {
            count: entity.0.count.unwrap() as u32,
            number_used_in_past_30_days: entity.0.number_used_in_past_thirty_days.unwrap() as u32,
            on_grocery_list: false,
            product: Some(Product {
                brand: product_entity.brand,
                category: product_entity.category,
                image_url: product_entity.image_url,
                label: product_entity.label.unwrap_or_default(),
                upc: product_entity.upc,
            }),
        };

        results.push(result);
    }

    results
}

pub async fn get_inventory_by_upc(db: &DatabaseConnection, upc: &String) -> InventoryItem {
    let entity = InventoryEntity::find()
        .find_also_related(ProductEntity)
        .filter(entity::products::Column::Upc.like(upc))
        .one(db)
        .await
        .ok()
        .unwrap()
        .unwrap();

    dbg!(&entity);

    let product_entity = entity.1.unwrap();

    InventoryItem {
        count: entity.0.count.unwrap() as u32,
        number_used_in_past_30_days: entity.0.number_used_in_past_thirty_days.unwrap() as u32,
        on_grocery_list: false,
        product: Some(Product {
            brand: product_entity.brand,
            category: product_entity.category,
            image_url: product_entity.image_url,
            label: product_entity.label.unwrap_or_default(),
            upc: product_entity.upc,
        }),
    }
}
