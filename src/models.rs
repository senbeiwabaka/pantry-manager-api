use rocket::response::Responder;
use rocket::{response, Request, Response};
use rocket_okapi::JsonSchema;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct Product {
    pub upc: String,
    pub label: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub image_url: Option<String>,
}

impl Clone for Product {
    fn clone(&self) -> Self {
        Product {
            upc: String::from(&self.upc),
            label: String::from(&self.label),
            brand: self.brand.clone(),
            category: self.category.clone(),
            image_url: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct InventoryItem {
    pub count: u32,
    pub number_used_in_past_30_days: u32,
    pub on_grocery_list: bool,
    pub product: Product,
}

// Model representing Groceries, Inventory, and Products.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, FromQueryResult)]
pub struct GroceryListItem {
    pub quantity: Option<u32>,
    pub shopped: Option<bool>,
    pub standard_quantity: Option<u32>,
    pub upc: String,
    pub label: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, FromQueryResult)]
pub struct InventoryItemProduct {
    pub upc: String,
    pub label: String,
    pub count: u32,
    pub inventory_item_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Paged<T> {
    pub count: u64,
    pub data: Vec<T>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct AppConfig {
    pub edaman_api_uri: String,
    pub edaman_app_id: String,
    pub edaman_app_key: String,
}

impl<'r> Responder<'r, 'static> for Product {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let model_as_string = serde_json::to_string(&self).unwrap();

        Response::build_from(model_as_string.respond_to(&req)?).ok()
    }
}

impl<'r> Responder<'r, 'static> for InventoryItem {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let model_as_string = serde_json::to_string(&self).unwrap();

        Response::build_from(model_as_string.respond_to(&req)?).ok()
    }
}
