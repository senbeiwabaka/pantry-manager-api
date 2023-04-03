use repository_db::Db;
use rocket::{http::Status, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    models::{InventoryItem, Paged, Product},
    services::inventory_services,
};

use repository::repositories::inventory_repository;

#[openapi]
#[get("/pantry-manager/inventory?<page>&<length>")]
pub async fn get_all_inventory(
    state: &State<Db>,
    page: Option<u64>,
    length: Option<u64>,
) -> Json<Paged<InventoryItem>> {
    let db = state.inner();
    let inventory = inventory_services::get_all_inventory(&db.conn, page, length).await;

    Json(inventory)
}

#[openapi]
#[get("/pantry-manager/inventory/<upc>")]
pub async fn get_inventory_by_upc(
    state: &State<Db>,
    upc: String,
) -> Result<Json<InventoryItem>, Status> {
    let db = state.inner();
    let exists = inventory_repository::exists(&db.conn, upc.clone()).await;

    if !exists {
        return Err(Status::NotFound);
    }

    let inventory = inventory_services::get_inventory_by_upc(&db.conn, &upc).await;

    Ok(Json(inventory))
}

#[openapi]
#[post("/pantry-manager/inventory", data = "<product>")]
pub async fn add_inventory_item(
    state: &State<Db>,
    product: Json<Product>,
) -> Result<(Status, Json<InventoryItem>), Status> {
    let db = state.inner();
    let exists = inventory_repository::exists(&db.conn, product.upc.clone()).await;

    if exists {
        return Err(Status::Conflict);
    }

    let inventory_item = inventory_services::add_inventory_item(&db.conn, &product, 1).await;

    Ok((Status::Created, Json(inventory_item)))
}

#[openapi]
#[put("/pantry-manager/inventory", data = "<inventory>")]
pub async fn update_inventory_item(
    state: &State<Db>,
    inventory: Json<InventoryItem>,
) -> Result<Json<InventoryItem>, Status> {
    let db = state.inner();
    let upc: String = inventory.product.clone().upc.clone();
    let exists = inventory_repository::exists(&db.conn, upc.clone()).await;

    if !exists {
        return Err(Status::NotFound);
    }

    inventory_services::update_inventory_item(&db.conn, &inventory).await;

    let inventory_item = inventory_services::get_inventory_by_upc(&db.conn, &upc).await;

    Ok(Json(inventory_item))
}

#[openapi]
#[post("/pantry-manager/inventory/<upc>/<count>")]
pub async fn update_inventory_count(
    state: &State<Db>,
    upc: String,
    count: i32,
) -> Result<Json<InventoryItem>, Status> {
    let db = state.inner();
    let exists = inventory_repository::exists(&db.conn, upc.clone()).await;

    if !exists {
        return Err(Status::NotFound);
    }

    if inventory_services::update_inventory_count(&db.conn, &upc, count).await {
        let inventory_item = inventory_services::get_inventory_by_upc(&db.conn, &upc).await;

        return Ok(Json(inventory_item));
    }

    Err(Status::InternalServerError)
}
