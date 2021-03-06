use repository_db::Db;
use rocket::{http::Status, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    models::{InventoryItem, Product},
    services::inventory_services,
};

use repository::repositories::inventory_repository;

#[openapi]
#[get("/pantry-manager/inventory")]
pub async fn get_all_inventory(state: &State<Db>) -> Json<Vec<InventoryItem>> {
    let db = state.inner();
    let inventory = inventory_services::get_all_inventory(&db.conn).await;

    dbg!(&inventory);

    Json(inventory)
}

#[openapi]
#[get("/pantry-manager/inventory/<upc>")]
pub async fn get_inventory_by_upc(state: &State<Db>, upc: String) -> Json<InventoryItem> {
    let db = state.inner();
    let inventory = inventory_services::get_inventory_by_upc(&db.conn, &upc).await;

    dbg!(&inventory);

    Json(inventory)
}

#[openapi]
#[post("/pantry-manager/inventory", data = "<product>")]
pub async fn add_inventory_item(
    state: &State<Db>,
    product: Json<Product>,
) -> Result<(Status, Json<InventoryItem>), Status> {
    let db = state.inner();

    let exists = inventory_repository::exists(&db.conn, &product.upc).await;

    if exists {
        return Err(Status::Conflict);
    }

    let inventory_item = inventory_services::add_inventory_item(&db.conn, &product, 1).await;
    let json_result = Json(inventory_item);

    // Ok(status::Created::new("created").body(json_result.into_inner()))

    Ok((Status::Created, json_result))
}

#[openapi]
#[put("/pantry-manager/inventory", data = "<inventory>")]
pub async fn update_inventory_item(state: &State<Db>, inventory: Json<InventoryItem>) -> Status {
    let db = state.inner();
    let upc: String = inventory.product.clone().unwrap().upc.clone();
    let exists = inventory_repository::exists(&db.conn, &upc).await;

    if !exists {
        return Status::Conflict;
    }

    let inventory_item = inventory_services::get_inventory_by_upc(&db.conn, &upc).await;

    inventory_services::update_inventory_item(&db.conn, &inventory_item).await;

    Status::NoContent
}
