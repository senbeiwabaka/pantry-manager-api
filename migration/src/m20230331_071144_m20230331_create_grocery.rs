use entity::inventory;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230331_071144_m20230331_create_grocery"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Grocery::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grocery::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Grocery::Quantity).integer())
                    .col(ColumnDef::new(Grocery::Shopped).boolean())
                    .col(ColumnDef::new(Grocery::StandardQuantity).integer())
                    .col(
                        ColumnDef::new(Grocery::InventoryItemId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed to create table");

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from_tbl(Grocery::Table)
                    .from_col(Grocery::InventoryItemId)
                    .to_tbl(inventory::Entity)
                    .to_col(inventory::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(sea_query::Index::drop().table(inventory::Entity).to_owned())
            .await
            .expect("Failed to drop index");

        manager
            .drop_table(sea_query::Table::drop().table(Grocery::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Grocery {
    Table,
    Id,
    Quantity,
    Shopped,
    StandardQuantity,
    InventoryItemId,
}
