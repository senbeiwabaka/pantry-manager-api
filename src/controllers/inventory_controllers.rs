use repository_db::Db;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use crate::{models::InventoryItem, services::inventory_services};

#[get("/pantry-manager/inventory")]
pub async fn get_all_inventory(conn: Connection<'_, Db>) -> Json<Vec<InventoryItem>> {
    let inventory = inventory_services::get_all_inventory(conn).await;

    dbg!(&inventory);

    Json(inventory)
}
