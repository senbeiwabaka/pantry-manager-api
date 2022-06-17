use rocket::{
    figment::{
        providers::{Env, Format, Toml},
        Profile,
    },
    request::{self, FromRequest},
    Config, Request,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub upc: String,
    pub label: String,
    pub brand: String,
    pub category: String,
    pub image_url: Option<String>,
}

impl Clone for Product {
    fn clone(&self) -> Self {
        Product {
            upc: String::from(&self.upc),
            label: String::from(&self.upc),
            brand: String::from(&self.upc),
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
    pub product: Product,
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

#[async_trait]
impl<'r> FromRequest<'r> for AppConfig {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let figment = request
            .rocket()
            .figment()
            .clone()
            .merge(Toml::file("Rocket.toml"))
            .merge(Env::prefixed("PANTRY_API_"))
            .select(Profile::from_env_or(
                "ROCKET_PROFILE",
                Config::DEBUG_PROFILE,
            ));

        let config: AppConfig = figment.focus("pantry_manager_api").extract().unwrap();

        request::Outcome::Success(config)
    }
}
