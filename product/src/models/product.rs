// Validation checks 👇
// description => min = 3
// price => should be greater than 0.1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
// use category::models::category::Category;
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    pub id: Option<RecordId>,
    pub name: String,
    pub(crate) description: String,
    pub price: f32,
    pub(crate) category: String,
    pub stock_quantity: u64,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: Option<DateTime<Utc>>,
}

// internally convert the Category-ID
#[derive(Deserialize, Validate)]
pub(crate) struct NewProduct {
    #[validate(length(min = 3))]
    pub(crate) name: String,
    #[validate(length(min = 3))]
    pub(crate) description: String,
    #[validate(range(exclusive_min = 0.1))]
    pub(crate) price: f32,
    pub(crate) category: String,
    pub(crate) stock_quantity: u64,
}

#[derive(Deserialize, Validate)]
pub(crate) struct UpdateProduct {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) price: Option<f32>,
    pub(crate) stock_quantity: Option<u64>,
}

#[derive(Deserialize)]
pub(crate) struct ProductStruct {
    pub(crate) name: String,
}
