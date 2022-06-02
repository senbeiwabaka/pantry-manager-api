use crate::models::{InventoryItem, Product};

pub fn add_inventory_item(product: &Product, count: u32) -> InventoryItem {
    InventoryItem {
        count: count,
        number_used_in_past_30_days: 0,
        on_grocery_list: false,
        product: product.clone(),
    }
}
