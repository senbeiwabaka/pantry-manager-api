//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub count: Option<i32>,
    pub number_used_in_past_thirty_days: Option<i32>,
    pub on_grocery_list: Option<bool>,
    pub product_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Products,
    #[sea_orm(has_many = "super::grocery::Entity")]
    Grocery,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl Related<super::grocery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grocery.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
