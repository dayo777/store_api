use crate::models::order::{NewOrder, Order, OrderID, OrderUpdateStatus};
use actix_web::{web, HttpResponse, Responder};

pub(crate) async fn create_new_order(new_order: web::Json<NewOrder>) -> impl Responder {
    match NewOrder::new_order(new_order.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Order successfully created!"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub(crate) async fn list_orders() -> impl Responder {
    match Order::list_orders().await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub(crate) async fn get_order(order_id: web::Json<OrderID>) -> impl Responder {
    match OrderID::get_order_id(order_id.into_inner()).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub(crate) async fn order_update_status(
    order_status: web::Json<OrderUpdateStatus>,
) -> impl Responder {
    match OrderUpdateStatus::update_status(order_status.into_inner()).await {
        Ok(order_status) => HttpResponse::Ok().json(order_status),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
