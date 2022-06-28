use rocket::response::Responder;
use rocket::{response, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub upc: String,
    pub label: String,
    pub brand: Option<String>,
    pub category: String,
    pub image_url: Option<String>,
}

impl Clone for Product {
    fn clone(&self) -> Self {
        Product {
            upc: String::from(&self.upc),
            label: String::from(&self.upc),
            brand: Some(String::from(&self.upc)),
            category: String::from(&self.upc),
            image_url: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct AppConfig {
    pub edaman_api_key: String,
}

impl<'r> Responder<'r, 'static> for Product {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let product_as_string = serde_json::to_string(&self).unwrap();

        Response::build_from(product_as_string.respond_to(&req)?).ok()
    }
}
