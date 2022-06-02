use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220525_141804_m20220525_1418_create_product_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Products::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Products::Upc)
                            .string()
                            .string_len(100)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Products::Label).string().string_len(100))
                    .col(ColumnDef::new(Products::Brand).string().string_len(100))
                    .col(ColumnDef::new(Products::Category).string().string_len(100))
                    .col(ColumnDef::new(Products::ImageUrl).string().string_len(100))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
    Upc,
    Label,
    Brand,
    Category,
    ImageUrl,
}
