use sea_orm::{entity::*, DatabaseConnection, QueryFilter};
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};

use crate::{models::Product, Db};

use entity::products;
use entity::products::Entity as ProductEntity;

pub async fn get_all_products(conn: Connection<'_, Db>) -> Vec<Product> {
    let db = conn.into_inner();
    let entities: Vec<products::Model> = ProductEntity::find().all(db).await.ok().unwrap();

    dbg!(&entities);

    let mut results: Vec<Product> = Vec::new();

    for entity in entities {
        let result = Product {
            brand: entity.brand,
            category: entity.category,
            image_url: entity.image_url,
            label: entity.label.unwrap(),
            upc: entity.upc,
        };

        results.push(result);
    }

    results
}

pub async fn add_product(db: &DatabaseConnection, product: &Product) {
    products::ActiveModel {
        brand: Set(product.brand.to_owned()),
        category: Set(product.category.to_owned()),
        label: Set(Some(product.label.to_owned())),
        upc: Set(product.upc.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("Failed to save new product");
}

pub async fn get_product_by_upc(db: &DatabaseConnection, upc: &String) -> Product {
    let product_entity = ProductEntity::find()
        .filter(products::Column::Upc.like(&upc))
        .one(db)
        .await
        .ok()
        .unwrap()
        .unwrap();

    dbg!(&product_entity);

    Product {
        brand: product_entity.brand,
        category: product_entity.category,
        image_url: product_entity.image_url,
        label: product_entity.label.unwrap(),
        upc: product_entity.upc,
    }
}

pub async fn lookup_product_by_upc(key: &String, upc: &String) -> Option<Product> {
    let request_url = format!(
        "https://edamam-food-and-grocery-database.p.rapidapi.com/parser?upc={}",
        upc
    );

    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header(
            "x-rapidapi-host",
            "edamam-food-and-grocery-database.p.rapidapi.com",
        )
        .header("x-rapidapi-key", key)
        // confirm the request using send()
        .send()
        .await
        .unwrap();

    let edaman_product: EdamamProduct;

    dbg!(&response);

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<EdamamProduct>().await {
                Ok(parsed) => {
                    println!("Success! {:?}", parsed);
                    edaman_product = parsed;
                }
                Err(error) => {
                    println!("error {}", error);
                    println!("Hm, the response didn't match the shape we expected.");

                    return None;
                }
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");

            return None;
        }
        other => {
            println!("Uh oh! Something unexpected happened: {:?}", other);

            return None;
        }
    }

    dbg!(&edaman_product);

    Some(Product {
        upc: String::from(upc),
        label: String::from(&edaman_product.hints[0].food.label),
        brand: edaman_product.hints[0].food.brand.to_owned(),
        category: Some(edaman_product.hints[0].food.category.to_owned()),
        image_url: edaman_product.hints[0].food.image.to_owned(),
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct EdamamProduct {
    text: String,
    hints: Vec<Hint>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hint {
    food: Food,
}

#[derive(Serialize, Deserialize, Debug)]
struct Food {
    label: String,
    brand: Option<String>,
    category: String,
    image: Option<String>,
}

impl Default for EdamamProduct {
    fn default() -> EdamamProduct {
        EdamamProduct {
            text: String::new(),
            hints: Vec::new(),
        }
    }
}
