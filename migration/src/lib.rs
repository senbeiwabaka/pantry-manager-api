pub use sea_orm_migration::prelude::*;

mod m20220525_141804_m20220525_1418_create_product_table;
mod m20220526_100904_m20220526_1008_create_inventory_table;
mod m20230331_071144_m20230331_create_grocery;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220525_141804_m20220525_1418_create_product_table::Migration),
            Box::new(m20220526_100904_m20220526_1008_create_inventory_table::Migration),
            Box::new(m20230331_071144_m20230331_create_grocery::Migration),
        ]
    }
}
