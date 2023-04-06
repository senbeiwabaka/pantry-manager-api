use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, Set,
};

use crate::models::{InventoryItem, Paged, Product};

use entity::inventory;
use entity::inventory::Entity as InventoryEntity;
use entity::products;
use entity::products::Entity as ProductEntity;

pub async fn add_inventory_item(
    db: &DatabaseConnection,
    product: &Product,
    count: u32,
    on_grocery_list: Option<bool>,
) -> InventoryItem {
    let product_entity = ProductEntity::find()
        .filter(products::Column::Upc.like(&product.upc.to_owned()))
        .one(db)
        .await
        .ok()
        .unwrap()
        .unwrap();
    let entity = inventory::ActiveModel {
        count: Set(Some(count as i32)),
        number_used_in_past_thirty_days: Set(Some(0)),
        on_grocery_list: Set(on_grocery_list),
        product_id: Set(product_entity.id),
        ..Default::default()
    };

    entity.save(db).await.unwrap();

    InventoryItem {
        count,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: product.clone(),
    }
}

pub async fn get_all_inventory(
    db: &DatabaseConnection,
    page: Option<u64>,
    length: Option<u64>,
) -> Paged<InventoryItem> {
    let count: usize = InventoryEntity::find()
        .find_also_related(ProductEntity)
        .count(db)
        .await
        .unwrap();
    let page_value = match page {
        Some(x) => x,
        _ => 0,
    };
    let langth_value = match length {
        Some(x) => x,
        _ => 10,
    };

    let entities = InventoryEntity::find()
        .find_also_related(ProductEntity)
        .limit(langth_value)
        .offset(page_value)
        .all(db)
        .await
        .unwrap();

    let mut results: Vec<InventoryItem> = Vec::new();

    for entity in entities {
        let product_entity = entity.1.unwrap();
        let result = InventoryItem {
            count: entity.0.count.unwrap() as u32,
            number_used_in_past_30_days: entity.0.number_used_in_past_thirty_days.unwrap() as u32,
            on_grocery_list: false,
            product: Product {
                brand: product_entity.brand,
                category: product_entity.category,
                image_url: product_entity.image_url,
                label: product_entity.label.unwrap_or_default(),
                upc: product_entity.upc,
            },
        };

        results.push(result);
    }

    let paged_data = Paged::<InventoryItem> {
        count,
        data: results,
    };

    paged_data
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

    let product_entity = entity.1.unwrap();

    InventoryItem {
        count: entity.0.count.unwrap() as u32,
        number_used_in_past_30_days: entity.0.number_used_in_past_thirty_days.unwrap() as u32,
        on_grocery_list: false,
        product: Product {
            brand: product_entity.brand,
            category: product_entity.category,
            image_url: product_entity.image_url,
            label: product_entity.label.unwrap_or_default(),
            upc: product_entity.upc,
        },
    }
}

pub async fn update_inventory_item(
    db: &DatabaseConnection,
    inventory_item: &InventoryItem,
) -> bool {
    let model: Option<inventory::Model> = InventoryEntity::find()
        .filter(entity::products::Column::Upc.like(inventory_item.product.upc.clone().as_str()))
        .one(db)
        .await
        .ok()
        .unwrap();

    let mut entity: inventory::ActiveModel = model.unwrap().into();

    entity.count = Set(Some(inventory_item.count as i32));

    let result = entity.update(db).await;

    match result {
        Ok(..) => true,
        _ => false,
    }
}

pub async fn update_inventory_count(db: &DatabaseConnection, upc: &String, count: i32) -> bool {
    let model = InventoryEntity::find()
        .find_also_related(ProductEntity)
        .filter(entity::products::Column::Upc.like(upc))
        .one(db)
        .await
        .ok()
        .unwrap();

    let mut entity: inventory::ActiveModel = model.unwrap().0.into();

    let mut new_count = match entity.count.unwrap() {
        Some(x) => x,
        _ => 0,
    } + count;

    if new_count < 0 {
        new_count = 0;
    }

    entity.count = Set(Some(new_count));

    let result = entity.update(db).await;

    match result {
        Ok(..) => true,
        _ => false,
    }
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32) {
    let entity: inventory::ActiveModel = InventoryEntity::find()
        .filter(entity::inventory::Column::Id.eq(id))
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    entity.delete(db).await.unwrap();
}
