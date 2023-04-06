use repository_db::Db;
use rocket::{http::Status, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    models::{GroceryListItem, InventoryItem, Paged},
    services::{grocery_services, inventory_services, product_services},
};

use repository::repositories::grocery_repository;

#[openapi]
#[get("/pantry-manager/groceries/all")]
pub async fn get_all_groceries(state: &State<Db>) -> Json<Paged<GroceryListItem>> {
    let db = state.inner();
    let data = grocery_services::get_all_groceries(&db.conn).await;

    Json(data)
}

#[openapi]
#[get("/pantry-manager/groceries?<page>&<length>")]
pub async fn get_groceries(
    state: &State<Db>,
    page: Option<u64>,
    length: Option<u64>,
) -> Json<Paged<GroceryListItem>> {
    let db = state.inner();
    let data = grocery_services::get_groceries(&db.conn, page, length).await;

    Json(data)
}

#[openapi]
#[get("/pantry-manager/groceries/shopping-list?<page>&<length>")]
pub async fn get_shopping_list(
    state: &State<Db>,
    page: Option<u64>,
    length: Option<u64>,
) -> Json<Paged<GroceryListItem>> {
    let db = state.inner();
    let data = grocery_services::get_shopping_list(&db.conn, page, length).await;

    Json(data)
}

#[openapi]
#[get("/pantry-manager/groceries/<upc>")]
pub async fn get_grocery_list_item(
    state: &State<Db>,
    upc: String,
) -> Result<(Status, Json<GroceryListItem>), Status> {
    let db = state.inner();
    let data = grocery_services::get_grocery_list_item(&db.conn, &upc).await;

    match data {
        Some(x) => Ok((Status::Ok, Json(x))),
        _ => Err(Status::NotFound),
    }
}

#[openapi]
#[post("/pantry-manager/groceries", data = "<inventory_item>")]
pub async fn post_add_inventory_item(
    state: &State<Db>,
    inventory_item: Json<InventoryItem>,
) -> Result<(Status, Json<GroceryListItem>), Status> {
    let db = state.inner();
    let exists = grocery_repository::exists(&db.conn, inventory_item.product.upc.clone()).await;

    if exists {
        return Err(Status::Conflict);
    }

    let grocery_list_item =
        grocery_services::add_grocery_list_item(&db.conn, &inventory_item.product.upc, 1, None)
            .await;

    Ok((Status::Created, Json(grocery_list_item)))
}

#[openapi]
#[post(
    "/pantry-manager/groceries/add-adhoc/<quantity>",
    data = "<inventory_item>"
)]
pub async fn post_add_adhoc(
    state: &State<Db>,
    quantity: i32,
    inventory_item: Json<InventoryItem>,
) -> Result<(Status, Json<GroceryListItem>), Status> {
    let db = state.inner();
    let exists = grocery_repository::exists(&db.conn, inventory_item.product.upc.clone()).await;

    if exists {
        return Err(Status::Conflict);
    }

    product_services::add_product(&db.conn, &inventory_item.product).await;

    let product =
        product_services::get_product_by_upc(&db.conn, &inventory_item.product.upc.clone()).await;

    inventory_services::add_inventory_item(
        &db.conn,
        &product,
        inventory_item.count,
        Some(inventory_item.on_grocery_list),
    )
    .await;

    let grocery_list_item = grocery_services::add_grocery_list_item(
        &db.conn,
        &inventory_item.product.upc,
        0,
        Some(quantity),
    )
    .await;

    Ok((Status::Created, Json(grocery_list_item)))
}

#[openapi]
#[post("/pantry-manager/groceries/standard-quantity/<upc>/<quantity>")]
pub async fn post_standard_quantity(
    state: &State<Db>,
    upc: String,
    quantity: u32,
) -> Result<Status, Status> {
    let db = state.inner();
    let exists = grocery_repository::exists(&db.conn, upc.clone()).await;

    dbg!(&exists);

    if !exists {
        return Err(Status::NotFound);
    }

    if grocery_services::set_standard_quantity(&db.conn, &upc, quantity).await {
        return Ok(Status::NoContent);
    }

    Err(Status::InternalServerError)
}

#[openapi]
#[post("/pantry-manager/groceries/shopping/done")]
pub async fn post_shopping_done(state: &State<Db>) -> Status {
    let db = state.inner();

    grocery_services::remove_adhoc_items(&db.conn).await;

    grocery_services::reset_items(&db.conn).await;

    Status::NoContent
}

#[openapi]
#[post("/pantry-manager/groceries/set-quantity/<upc>/<quantity>")]
pub async fn post_set_quantity(state: &State<Db>, upc: String, quantity: u32) -> Status {
    let db = state.inner();
    let exists = grocery_repository::exists(&db.conn, upc.clone()).await;

    dbg!(&exists);

    if !exists {
        return Status::NotFound;
    }

    grocery_services::set_quantity(&db.conn, &upc, quantity).await;

    Status::NoContent
}
