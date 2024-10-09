use crate::models::category::Category;
use chrono::{DateTime, Utc};
use database::DB;

pub(crate) fn get_datetime() -> DateTime<Utc> {
    Utc::now()
}

pub async fn check_if_category_exist(name: String) -> bool {
    let mut response = DB
        .query("SELECT * FROM category where name = $name")
        .bind(("name", name.to_lowercase()))
        .await
        .expect("Error checking if_category exists.");

    let response: Option<Category> = response.take(0).unwrap();
    if let Some(_x) = response {
        return true;
    }

    false
}
