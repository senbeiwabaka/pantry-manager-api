use repository_db::Db;
use rocket::{http::Status, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    models::{GroceryListItem, Paged},
    services::grocery_services,
};

use repository::repositories::grocery_repository;

#[openapi]
#[get("/pantry-manager/groceries?<page>&<length>")]
pub async fn get_all_groceries(
    state: &State<Db>,
    page: Option<u64>,
    length: Option<u64>,
) -> Json<Paged<GroceryListItem>> {
    let db = state.inner();
    let data = grocery_services::get_all_groceries(&db.conn, page, length).await;

    Json(data)
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

    

    Ok(Status::NoContent)
}
