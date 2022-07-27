use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use entity::inventory;
use entity::inventory::Entity as InventoryItemEntity;
use entity::products;
use entity::products::Entity as ProductEntity;

pub async fn exists(db: &DatabaseConnection, upc: &String) -> bool {
    let product_entity = ProductEntity::find()
        .filter(products::Column::Upc.like(&upc))
        .one(db)
        .await
        .unwrap_or_default();

    match product_entity {
        Some(_pp) => {
            let entity = InventoryItemEntity::find()
                .filter(inventory::Column::ProductId.eq(_pp.id))
                .one(db)
                .await
                .unwrap_or_default();

            match entity {
                Some(_pp) => true,
                _ => false,
            }
        }
        _ => false,
    }
}
