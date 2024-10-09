use crate::models::product::Product;
use chrono::{DateTime, Utc};
use database::DB;

pub(crate) fn get_datetime() -> DateTime<Utc> {
    Utc::now()
}

pub(crate) async fn check_if_product_exist(name: String) -> bool {
    let mut response = DB
        .query("SELECT * FROM product WHERE name = $name")
        .bind(("name", name.to_lowercase()))
        .await
        .expect("Error checking duplicate Product name.");

    let response: Option<Product> = response.take(0).expect("Error retrieving product name.");
    if response.is_some() {
        return true;
    }
    false
}
