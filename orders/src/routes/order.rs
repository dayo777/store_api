use crate::models::order::{NewOrder, Order, OrderID, OrderUpdateStatus};
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};

#[tracing::instrument(skip(new_order), fields(log_name))]
pub(crate) async fn create_new_order(new_order: web::Json<NewOrder>) -> impl Responder {
    let log_name = new_order.0.customer_email.clone();
    match NewOrder::new_order(new_order.into_inner()).await {
        Ok(_) => {
            info!("Created new order route: {}", log_name);
            HttpResponse::Ok().json("Order successfully created!")
        }
        Err(e) => {
            error!("Error creating new order route: {}", log_name);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

#[tracing::instrument()]
pub(crate) async fn list_orders() -> impl Responder {
    match Order::list_orders().await {
        Ok(orders) => {
            info!("Returned orders list: {:?}", orders.len());
            HttpResponse::Ok().json(orders)
        }
        Err(e) => {
            error!("Error listing orders route: {}", e);
            HttpResponse::NotFound().json(e.to_string())
        }
    }
}

#[tracing::instrument(skip(order_id), fields(log_name))]
pub(crate) async fn get_order(order_id: web::Json<OrderID>) -> impl Responder {
    let log_name = order_id.0.customer_email.clone();
    match OrderID::get_order_id(order_id.into_inner()).await {
        Ok(order) => {
            info!("Retrieving order route: {}", log_name);
            HttpResponse::Ok().json(order)
        }
        Err(e) => {
            error!("Error getting order route: {}", log_name);
            HttpResponse::NotFound().json(e.to_string())
        }
    }
}

#[tracing::instrument(skip(order_status), fields(log_name))]
pub(crate) async fn order_update_status(
    order_status: web::Json<OrderUpdateStatus>,
) -> impl Responder {
    let log_name = order_status.0.customer_email.clone();
    match OrderUpdateStatus::update_status(order_status.into_inner()).await {
        Ok(order_status) => {
            info!("Updating order status route: {}", log_name);
            HttpResponse::Ok().json(order_status)
        }
        Err(e) => {
            error!("Error updating order status route: {}", log_name);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}
