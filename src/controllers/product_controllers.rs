use repository_db::Db;
use rocket::{
    response::status::{self, Conflict, Created, NoContent, NotFound},
    serde::json::Json,
};
use sea_orm_rocket::Connection;

use crate::{models::Product, services::product_services};

use repository::repositories::product_repository;

#[get("/pantry-manager/products")]
pub async fn get_all_products(conn: Connection<'_, Db>) -> Json<Vec<Product>> {
    let products = product_services::get_all_products(conn).await;

    dbg!(&products);

    Json(products)
}

#[get("/pantry-manager/product/<upc>")]
pub async fn get_product(
    conn: Connection<'_, Db>,
    upc: String,
) -> Result<Json<Product>, NotFound<String>> {
    let db = conn.into_inner();
    let exists = product_repository::exists(&db, upc.clone()).await;

    if !exists {
        return Err(status::NotFound("Not Found".to_string()));
    }

    let product = product_services::get_product_by_upc(&db, &upc).await;

    dbg!(&product);

    Ok(Json(product))
}

#[post("/pantry-manager/product", data = "<product>")]
pub async fn add_product(
    conn: Connection<'_, Db>,
    product: Json<Product>,
) -> Result<Created<Product>, Conflict<String>> {
    let db = conn.into_inner();
    let exists = product_repository::exists(&db, product.upc.clone()).await;

    if exists {
        return Err(status::Conflict(Some("product already exists".to_string())));
    }

    product_services::add_product(&db, &product).await;

    Ok(status::Created::new("created").body(product.into_inner()))
}

#[delete("/pantry-manager/product/<upc>")]
pub async fn remove_product(
    conn: Connection<'_, Db>,
    upc: String,
) -> Result<NoContent, NotFound<String>> {
    let db = conn.into_inner();
    let exists = product_repository::exists(&db, upc).await;

    if !exists {
        return Err(status::NotFound("product does not exist".to_string()));
    }

    Ok(status::NoContent)
}
