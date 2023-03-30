use rocket::response::Responder;
use rocket::{response, Request, Response};
use rocket_okapi::JsonSchema;
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
    pub product: Option<Product>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroceryListItem {
    pub quantity: u32,
    pub shopped: bool,
    pub standard_quantity: u32,
    pub inventory_item: InventoryItem,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Paged<T> {
    pub count: usize,
    pub data: Vec<T>,
}

impl<T> Paged<T> {
    pub const fn new() -> Self {
        Paged {
            count: 0,
            data: Vec::new(),
        }
    }
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
