use repository_db::Db;
use rocket::{serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    models::{GroceryListItem, Paged},
    services::grocery_services,
};

// use repository::repositories::inventory_repository;

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
