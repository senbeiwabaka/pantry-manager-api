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

use repository::repositories::inventory_repository;

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

#[post("/pantry-manager/inventory/<count>", data = "<product>")]
pub async fn add_inventory_item(
    conn: Connection<'_, Db>,
    count: u32,
    product: Json<Product>,
) -> Result<Created<InventoryItem>, Conflict<String>> {
    let db = conn.into_inner();

    let exists = inventory_repository::exists(&db, product.upc.clone()).await;

    if exists {
        return Err(status::Conflict(Some(
            "inventory already exists".to_string(),
        )));
    }

    let inventory_item = inventory_services::add_inventory_item(&db, &product, count).await;
    let json_result = Json(inventory_item);

    Ok(status::Created::new("created").body(json_result.into_inner()))
}
