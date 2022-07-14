use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use entity::products;
use entity::products::Entity as ProductEntity;

pub async fn exists(db: &DatabaseConnection, upc: String) -> bool {
    let entity = ProductEntity::find()
        .filter(products::Column::Upc.like(&upc))
        .one(db)
        .await;

    match entity {
        Ok(p) => match p {
            Some(_pp) => true,
            _ => false,
        },
        _ => false,
    }
}