use crate::models::product::{NewProduct, Product, ProductStruct, UpdateProduct};
use actix_web::{web, HttpResponse, Responder};

pub(crate) async fn create_product(new_product: web::Json<NewProduct>) -> impl Responder {
    match NewProduct::create_new_product(new_product.into_inner()).await {
        Ok(e) => HttpResponse::Created().json(e),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub(crate) async fn list_product() -> impl Responder {
    match Product::list_products().await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub(crate) async fn update_product(update_product: web::Json<UpdateProduct>) -> impl Responder {
    match UpdateProduct::update_product(update_product.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("Product update successful!".to_string()),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub(crate) async fn get_product(product_name: web::Json<ProductStruct>) -> impl Responder {
    match ProductStruct::get_product(product_name.into_inner()).await {
        Ok(e) => HttpResponse::Ok().json(e),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}
