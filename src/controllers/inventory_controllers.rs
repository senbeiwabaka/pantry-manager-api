use repository_db::Db;
use rocket::{
    response::status::{self, Conflict, Created},
    serde::json::Json,
};
use sea_orm_rocket::Connection;

use crate::{
    models::{InventoryItem, Product},
    services::inventory_services,
};

#[get("/pantry-manager/inventory")]
pub async fn get_all_inventory(conn: Connection<'_, Db>) -> Json<Vec<InventoryItem>> {
    let db = conn.into_inner();
    let inventory = inventory_services::get_all_inventory(db).await;

    dbg!(&inventory);

    Json(inventory)
}

#[get("/pantry-manager/inventory/<upc>")]
pub async fn get_inventory_by_upc(conn: Connection<'_, Db>, upc: String) -> Json<InventoryItem> {
    let db = conn.into_inner();
    let inventory = inventory_services::get_inventory_by_upc(&db, &upc).await;

    dbg!(&inventory);

    Json(inventory)
}

#[post("/pantry-manager/inventory/<count>")]
pub async fn add_inventory_item(
    conn: Connection<'_, Db>,
    count: u32,
) -> Result<Created<String>, Conflict<String>> {
    let product = Product {
        brand: None,
        category: None,
        image_url: None,
        label: "".to_string(),
        upc: "".to_string(),
    };

    let db = conn.into_inner();
    inventory_services::add_inventory_item(&db, &product, count).await;

    Ok(status::Created::new("created").body("test".to_string()))
}
