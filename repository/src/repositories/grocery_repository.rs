use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

use entity::grocery;
use entity::grocery::Entity as GroceryEntity;

pub async fn exists(db: &DatabaseConnection, upc: String) -> bool {
    let entity = GroceryEntity::find()
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .filter(entity::products::Column::Upc.like(&upc))
        .one(db)
        .await
        .unwrap_or_default();

    dbg!(&entity);

    match entity {
        Some(_pp) => true,
        _ => false,
    }
}
