use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

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

    match entity {
        Some(_pp) => true,
        _ => false,
    }
}

pub async fn get_all_adhoc(db: &DatabaseConnection) -> Vec<entity::grocery::Model> {
    let entities = GroceryEntity::find()
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .filter(entity::products::Column::Brand.is_null())
        .all(db)
        .await
        .unwrap_or_default();

    entities
}

pub async fn get_all(db: &DatabaseConnection) -> Vec<entity::grocery::Model> {
    let entities = GroceryEntity::find()
        .join(
            JoinType::LeftJoin,
            entity::grocery::Relation::Inventory.def(),
        )
        .join(
            JoinType::LeftJoin,
            entity::inventory::Relation::Products.def(),
        )
        .filter(entity::products::Column::Brand.is_not_null())
        .all(db)
        .await
        .unwrap_or_default();

    entities
}
