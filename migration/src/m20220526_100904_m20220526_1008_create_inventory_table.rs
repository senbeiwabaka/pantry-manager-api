use sea_orm_migration::prelude::*;

use entity::products;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220526_100904_m20220526_1008_create_inventory_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Inventory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Inventory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Inventory::Count).integer())
                    .col(ColumnDef::new(Inventory::NumberUsedInPastThirtyDays).integer())
                    .col(ColumnDef::new(Inventory::OnGroceryList).boolean())
                    .col(ColumnDef::new(Inventory::ProductId).integer().not_null())
                    .to_owned(),
            )
            .await
            .expect("Failed to create table");

        manager
            .create_index(
                sea_query::Index::create()
                    .name("idx-products-upc")
                    .table(products::Entity)
                    .col(products::Column::Upc)
                    .to_owned(),
            )
            .await
            .expect("Failed to create product index");

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from_tbl(Inventory::Table)
                    .from_col(Inventory::ProductId)
                    .to_tbl(products::Entity)
                    .to_col(products::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Inventory::Table).to_owned())
            .await
            .expect("Failed to drop table");

        manager
            .drop_index(sea_query::Index::drop().table(products::Entity).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Inventory {
    Table,
    Id,
    Count,
    NumberUsedInPastThirtyDays,
    OnGroceryList,
    ProductId,
}
