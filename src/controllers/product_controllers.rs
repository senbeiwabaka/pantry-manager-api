use repository_db::Db;
use rocket::{
    http::Status,
    response::status::{self, NoContent, NotFound},
    serde::json::Json,
    State,
};
use rocket_okapi::openapi;

use crate::{models::Product, services::product_services};

use repository::repositories::product_repository;

#[openapi]
#[get("/pantry-manager/products")]
pub async fn get_all_products(state: &State<Db>) -> Json<Vec<Product>> {
    let db = state.inner();
    let products = product_services::get_all_products(&db.conn).await;

    dbg!(&products);

    Json(products)
}

#[openapi]
#[get("/pantry-manager/product/<upc>")]
pub async fn get_product(state: &State<Db>, upc: String) -> Result<Json<Product>, Status> {
    let db = state.inner();
    let exists = product_repository::exists(&db.conn, upc.clone()).await;

    if !exists {
        return Err(Status::NotFound);
    }

    let product = product_services::get_product_by_upc(&db.conn, upc).await;

    dbg!(&product);

    Ok(Json(product))
}

#[openapi]
#[post("/pantry-manager/product", data = "<product>")]
pub async fn add_product(
    state: &State<Db>,
    product: Json<Product>,
) -> Result<(Status, Json<Product>), Status> {
    let db = state.inner();
    let exists = product_repository::exists(&db.conn, product.upc.clone()).await;

    if exists {
        // return Err(status::Conflict(Some("product already exists".to_string())));

        return Err(Status::Conflict);
    }

    product_services::add_product(&db.conn, &product).await;

    // status::Created::new("created").body(product.into_inner())

    Ok((Status::Created, product))
}

#[openapi]
#[delete("/pantry-manager/product/<upc>")]
pub async fn remove_product(state: &State<Db>, upc: String) -> Result<NoContent, NotFound<String>> {
    let db = state.inner();
    let exists = product_repository::exists(&db.conn, upc).await;

    if !exists {
        return Err(status::NotFound("product does not exist".to_string()));
    }

    Ok(status::NoContent)
}
