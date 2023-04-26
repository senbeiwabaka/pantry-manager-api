use migration::JoinType;
use repository::repositories::{grocery_repository, inventory_repository};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, RelationTrait, Set,
};

use crate::models::{GroceryListItem, InventoryItemProduct, Paged};

use entity::grocery::Entity as GroceryEntity;
use entity::inventory::Entity as InventoryEntity;
use entity::products::Entity as ProductEntity;
use entity::{grocery, inventory};

use super::{inventory_services, product_services};

pub async fn get_all_groceries(db: &DatabaseConnection) -> Vec<GroceryListItem> {
    let entities = GroceryEntity::find()
        .select_only()
        .column(entity::products::Column::Upc)
        .column(entity::products::Column::Label)
        .column_as(
            entity::grocery::Column::StandardQuantity.if_null(0 as u32),
            "standard_quantity",
        )
        .column_as(entity::grocery::Column::Shopped.if_null(false), "shopped")
        .column_as(
            entity::grocery::Column::Quantity.if_null(0 as u32),
            "quantity",
        )
        .column_as(entity::inventory::Column::Count.if_null(0 as u32), "count")
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .into_model::<GroceryListItem>()
        .all(db)
        .await
        .unwrap();

    entities
}

pub async fn get_groceries(
    db: &DatabaseConnection,
    page: Option<u64>,
    length: Option<u64>,
) -> Paged<GroceryListItem> {
    let count: usize = GroceryEntity::find()
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .select_only()
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

    let entities = GroceryEntity::find()
        .select_only()
        .column(entity::products::Column::Upc)
        .column(entity::products::Column::Label)
        .column_as(
            entity::grocery::Column::StandardQuantity.if_null(0 as u32),
            "standard_quantity",
        )
        .column_as(entity::grocery::Column::Shopped.if_null(false), "shopped")
        .column_as(
            entity::grocery::Column::Quantity.if_null(0 as u32),
            "quantity",
        )
        .column_as(entity::inventory::Column::Count.if_null(0 as u32), "count")
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .limit(langth_value)
        .offset(page_value)
        .into_model::<GroceryListItem>()
        .all(db)
        .await
        .unwrap();

    let paged_data = Paged::<GroceryListItem> {
        count,
        data: entities,
    };

    paged_data
}

// Get grocery items where they are marked to be on the list (boolean) and their quantity wanted is greater than 0
pub async fn get_shopping_list(
    db: &DatabaseConnection,
    page: Option<u64>,
    length: Option<u64>,
) -> Paged<GroceryListItem> {
    let count: usize = GroceryEntity::find()
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .select_only()
        .filter(entity::inventory::Column::OnGroceryList.eq(true))
        .filter(entity::grocery::Column::Quantity.gt(0))
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

    let entities = GroceryEntity::find()
        .select_only()
        .column(entity::products::Column::Upc)
        .column(entity::products::Column::Label)
        .column_as(
            entity::grocery::Column::StandardQuantity.if_null(0 as u32),
            "standard_quantity",
        )
        .column_as(entity::grocery::Column::Shopped.if_null(false), "shopped")
        .column_as(
            entity::grocery::Column::Quantity.if_null(0 as u32),
            "quantity",
        )
        .column_as(entity::inventory::Column::Count.if_null(0 as u32), "count")
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .filter(entity::inventory::Column::OnGroceryList.eq(true))
        .filter(entity::grocery::Column::Quantity.gt(0))
        .limit(langth_value)
        .offset(page_value)
        .into_model::<GroceryListItem>()
        .all(db)
        .await
        .unwrap();

    let paged_data = Paged::<GroceryListItem> {
        count,
        data: entities,
    };

    paged_data
}

pub async fn get_grocery_list_item(
    db: &DatabaseConnection,
    upc: &String,
) -> Option<GroceryListItem> {
    let entity = GroceryEntity::find()
        .filter(entity::products::Column::Upc.like(upc))
        .select_only()
        .column(entity::products::Column::Upc)
        .column(entity::products::Column::Label)
        .column_as(
            entity::grocery::Column::StandardQuantity.if_null(0 as u32),
            "standard_quantity",
        )
        .column_as(entity::grocery::Column::Shopped.if_null(false), "shopped")
        .column_as(
            entity::grocery::Column::Quantity.if_null(0 as u32),
            "quantity",
        )
        .column_as(entity::inventory::Column::Count.if_null(0 as u32), "count")
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .into_model::<GroceryListItem>()
        .one(db)
        .await
        .unwrap();

    entity
}

pub async fn add_grocery_list_item(
    db: &DatabaseConnection,
    upc: &String,
    standard_quantity: u32,
    quantity: Option<i32>,
) -> GroceryListItem {
    let invetory_item_entity = InventoryEntity::find()
        .filter(entity::products::Column::Upc.like(upc))
        .select_only()
        .column(entity::products::Column::Upc)
        .column(entity::products::Column::Label)
        .column_as(entity::inventory::Column::Count.if_null(0 as u32), "count")
        .column_as(entity::inventory::Column::Id, "inventory_item_id")
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .into_model::<InventoryItemProduct>()
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let entity = grocery::ActiveModel {
        shopped: Set(Some(false)),
        standard_quantity: Set(Some(standard_quantity as i32)),
        quantity: Set(quantity),
        inventory_item_id: Set(invetory_item_entity.inventory_item_id),
        ..Default::default()
    };

    entity.save(db).await.unwrap();

    GroceryListItem {
        quantity: None,
        shopped: Some(false),
        standard_quantity: Some(standard_quantity),
        upc: upc.to_owned(),
        label: invetory_item_entity.label.to_owned(),
        count: invetory_item_entity.count,
    }
}

pub async fn set_standard_quantity(
    db: &DatabaseConnection,
    upc: &String,
    standard_quantity: u32,
) -> bool {
    let product_entity = ProductEntity::find()
        .filter(entity::products::Column::Upc.like(upc))
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let inventory_item_model = InventoryEntity::find()
        .filter(entity::inventory::Column::ProductId.eq(product_entity.id))
        .one(db)
        .await
        .unwrap()
        .unwrap();

    let mut inventory_item_entity: inventory::ActiveModel = inventory_item_model.clone().into();

    if standard_quantity > 0 {
        inventory_item_entity.on_grocery_list = Set(Some(true));
    } else {
        inventory_item_entity.on_grocery_list = Set(Some(false));
    }

    let result = inventory_item_entity.save(db).await;

    if result.is_err() {
        return false;
    }

    let mut entity: grocery::ActiveModel = GroceryEntity::find()
        .filter(entity::grocery::Column::InventoryItemId.eq(inventory_item_model.id))
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    entity.standard_quantity = Set(Some(standard_quantity as i32));

    let result = entity.update(db).await;

    match result {
        Ok(..) => true,
        _ => false,
    }
}

pub async fn remove_adhoc_items(db: &DatabaseConnection) {
    let models = grocery_repository::get_all_adhoc(&db).await;

    for model in models {
        let entity: grocery::ActiveModel = model.into();
        let inventory_id = entity.inventory_item_id.clone().unwrap();
        let inventory_item = inventory_repository::get_by_id(&db, inventory_id).await;
        let product_id = inventory_item.product_id.clone();

        delete_by_id(&db, entity.id.unwrap()).await;

        inventory_services::delete_by_id(&db, inventory_id).await;

        product_services::delete_by_id(&db, product_id).await;
    }
}

pub async fn reset_items(db: &DatabaseConnection) {
    let models = grocery_repository::get_all(&db).await;

    for model in models {
        let mut entity: grocery::ActiveModel = model.into();

        entity.quantity = Set(entity.standard_quantity.clone().unwrap());

        entity.update(db).await.unwrap();
    }
}

pub async fn set_quantity(db: &DatabaseConnection, upc: &String, quantity: u32) -> bool {
    let product_entity = ProductEntity::find()
        .filter(entity::products::Column::Upc.like(upc))
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let inventory_item_model = InventoryEntity::find()
        .filter(entity::inventory::Column::ProductId.eq(product_entity.id))
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut entity: grocery::ActiveModel = GroceryEntity::find()
        .filter(entity::grocery::Column::InventoryItemId.eq(inventory_item_model.id))
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    entity.quantity = Set(Some(quantity as i32));

    let result = entity.update(db).await;

    match result {
        Ok(..) => true,
        _ => false,
    }
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32) {
    let entity: grocery::ActiveModel = GroceryEntity::find()
        .filter(entity::grocery::Column::Id.eq(id))
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .into();

    entity.delete(db).await.unwrap();
}
