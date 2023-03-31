use migration::JoinType;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, RelationTrait,
};

use crate::models::{GroceryListItem, Paged};

use entity::grocery::Entity as GroceryEntity;

pub async fn get_all_groceries(
    db: &DatabaseConnection,
    page: Option<u64>,
    length: Option<u64>,
) -> Paged<GroceryListItem> {
    let count: usize = GroceryEntity::find()
        .join(
            JoinType::RightJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
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
        .join(
            JoinType::RightJoin,
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

    // let mut results: Vec<GroceryListItem> = Vec::new();

    // for entity in entities {
    //     let grocery_item = entity.0.to_owned();
    //     let inventor_item = entity.1.to_owned();

    //     let result = GroceryListItem {
    //         quantity: grocery_item.quantity.unwrap() as u32,
    //         shopped: grocery_item.shopped.unwrap_or_default(),
    //         standard_quantity: grocery_item.standard_quantity.unwrap() as u32,
    //         inventory_item: match inventor_item {
    //             Some(x) => Some(InventoryItem {
    //                 count: x.count.unwrap() as u32,
    //                 number_used_in_past_30_days: x.number_used_in_past_thirty_days.unwrap() as u32,
    //                 on_grocery_list: false,
    //                 product: None,
    //             }),
    //             _ => None,
    //         },
    //     };

    //     results.push(result);
    // }

    let paged_data = Paged::<GroceryListItem> {
        count,
        data: entities,
    };

    paged_data
}
