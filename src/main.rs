#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod controllers;
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

use rocket_cors::AllowedOrigins;
use sea_orm_rocket::Database;

use crate::{
    controllers::inventory_controllers, controllers::product_controllers, models::Product,
    services::product_services,
};

#[launch]
fn rocket() -> _ {
    let figment = Figment::new()
        .merge(Toml::file("Rocket.toml"))
        .merge(Env::prefixed("PANTRY_API_"))
        .select(Profile::from_env_or(
            "ROCKET_PROFILE",
            Config::DEBUG_PROFILE,
        ));

    dbg!(&figment);

    let config: AppConfig = figment.focus("pantry_manager_api").extract().unwrap();

    dbg!(&config);

    let allowed_origins = AllowedOrigins::All;
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    dbg!(&cors);

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(AdHoc::on_ignite(
            "Application Config",
            |rocket| async move { rocket.manage(config) },
        ))
        .attach(cors)
        .mount(
            "/",
            routes![
                upc,
                product_controllers::get_all_products,
                product_controllers::add_product,
                product_controllers::remove_product,
                product_controllers::get_product,
                inventory_controllers::get_all_inventory,
                inventory_controllers::get_inventory_by_upc,
                inventory_controllers::add_inventory_item,
            ],
        )
}

#[get("/pantry-manager/upc-lookup/<upc>")]
async fn upc(state: &State<AppConfig>, upc: String) -> Result<Json<Product>, NotFound<String>> {
    let app_config = state.inner();
    let result = product_services::lookup_product_by_upc(&app_config.edaman_api_key, &upc).await;
    let product: Product;

    match result {
        Some(p) => product = p,
        None => return Err(status::NotFound("Not Found".to_string())),
    }

    dbg!(&product);

    Ok(Json(product))
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;

    Ok(rocket)
}
