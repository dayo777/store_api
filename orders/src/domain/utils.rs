use crate::error::OrderError;
use crate::models::order::{CreateOrderItem, OrderItem};
use chrono::{DateTime, Utc};
use database::DB;
use product::models::product::Product;

// deduct total quantity purchased from Product table inventory
pub(crate) async fn deduct_from_stock_quantity(
    order_quantity: u64,
    product_name: String,
) -> Result<bool, OrderError> {
    let sql = "UPDATE product SET stock_quantity -= $order_quantity WHERE (name = $name) AND (stock_quantity > $order_quantity)";

    let mut response = DB
        // .query("UPDATE product SET stock_quantity -= $order_quantity WHERE name = $name")
        .query(sql)
        .bind(("order_quantity", order_quantity))
        .bind(("name", product_name))
        .await
        .map_err(OrderError::InternalError)?;

    let response: Option<Product> = response.take(0).expect("DB error");
    if response.is_none() {
        return Ok(false);
    }
    Ok(true)
}

// takes in a vector of OrderItems, and deducts each Order quantity from Product inventory
pub(crate) async fn deduct_batch_product(order_item: Vec<OrderItem>) -> Result<(), OrderError> {
    for item in order_item {
        match deduct_from_stock_quantity(item.quantity, item.product.name).await {
            Ok(value) => {
                if value {
                    continue;
                } else {
                    return Err(OrderError::InvalidOrder);
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

pub(crate) async fn construct_order_items(
    order_items: Vec<CreateOrderItem>,
) -> Result<Vec<OrderItem>, OrderError> {
    // create a Vector for each vector type
    let product_names = order_items
        .iter()
        .map(|p| p.product_name.clone())
        .collect::<Vec<String>>();

    let quantities = order_items.iter().map(|q| q.quantity).collect::<Vec<u64>>();

    let mut products = Vec::<Product>::new();
    for pn in &product_names {
        let mut resp = DB
            .query("SELECT * FROM product WHERE name = $pn")
            .bind(("pn", pn.to_lowercase()))
            .await?;
        let pn_result: Option<Product> = resp.take(0)?;
        match pn_result {
            Some(pn_result) => {
                products.push(pn_result);
            }
            None => return Err(OrderError::InvalidOrder),
        }
    }

    if (products.len() != product_names.len()) || (products.len() != quantities.len()) {
        return Err(OrderError::InvalidOrder);
    }

    let order_items: Vec<OrderItem> = products
        .into_iter()
        .zip(quantities.into_iter())
        .map(|(product, quantity)| OrderItem { product, quantity })
        .collect();

    println!("{:?}", order_items.clone());
    Ok(order_items)
}

// get the total-amount
pub(crate) fn calculate_total_price(order_items: Vec<OrderItem>) -> f32 {
    let total_items = order_items.iter().fold(0.0, |acc, item| {
        acc + (item.product.price * item.quantity as f32)
    });

    total_items
}

pub(crate) fn get_datetime() -> DateTime<Utc> {
    Utc::now()
}
