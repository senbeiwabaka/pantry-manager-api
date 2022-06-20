#[macro_use]
extern crate rocket;

mod models;
mod services;

use migration::MigratorTrait;
use models::AppConfig;
use repository_db::Db;
use rocket::{
    fairing::{self, AdHoc},
    figment::{
        providers::{Env, Format, Toml},
        Figment, Profile,
    },
    response::status::{self, *},
    serde::json::Json,
    Build, Config, Rocket, State,
};

use sea_orm_rocket::{Connection, Database};

use crate::{models::Product, services::product_services};

#[launch]
fn rocket() -> _ {
    let figment = Figment::new()
        .merge(Toml::file("Rocket.toml"))
        .merge(Env::prefixed("PANTRY_API_"))
        .select(Profile::from_env_or(
            "ROCKET_PROFILE",
            Config::DEBUG_PROFILE,
        ));

    let config: AppConfig = figment.focus("pantry_manager_api").extract().unwrap();

    dbg!(&config);

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(AdHoc::on_ignite(
            "Application Config",
            |rocket| async move { rocket.manage(config) },
        ))
        .mount("/", routes![upc, get_all_products, add_product])
}

#[get("/pantry-manager/upc-lookup/<upc>")]
async fn upc(state: &State<AppConfig>, upc: String) -> Result<Json<Product>, NotFound<String>> {
    let app_config = state.inner();
    let result = product_services::get_product_by_upc(&app_config.edaman_api_key, &upc).await;
    let product: Product;

    match result {
        Some(p) => product = p,
        None => return Err(status::NotFound("Not Found".to_string())),
    }

    dbg!(&product);

    Ok(Json(product))
}

#[get("/pantry-manager/products")]
async fn get_all_products(conn: Connection<'_, Db>) -> Json<Vec<Product>> {
    let products = product_services::get_all_products(conn).await;

    dbg!(&products);

    Json(products)
}

#[post("/pantry-manager/product", data = "<product>")]
async fn add_product(
    conn: Connection<'_, Db>,
    product: Json<Product>,
) -> Result<Created<String>, Conflict<String>> {
    let db = conn.into_inner();
    let exists = repository::exists(&db, product.upc.clone()).await;

    if exists {
        return Err(status::Conflict(Some("product already exists".to_string())));
    }

    product_services::add_product(&db, &product).await;

    Ok(status::Created::new("created"))
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;

    Ok(rocket)
}
