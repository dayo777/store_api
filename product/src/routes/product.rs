use crate::models::product::{NewProduct, Product, ProductStruct, UpdateProduct};
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};

#[tracing::instrument(skip(new_product), fields(log_name))]
pub(crate) async fn create_product(new_product: web::Json<NewProduct>) -> impl Responder {
    let log_name = new_product.0.name.clone();
    match NewProduct::create_new_product(new_product.into_inner()).await {
        Ok(e) => {
            info!("Created new product route: {:?}", log_name);
            HttpResponse::Created().json(e)
        }
        Err(e) => {
            error!("Error creating new product route: {:?}", log_name);
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

#[tracing::instrument()]
pub(crate) async fn list_product() -> impl Responder {
    match Product::list_products().await {
        Ok(products) => {
            info!("List products route: {:?}", products.len());
            HttpResponse::Ok().json(products)
        }
        Err(e) => {
            error!("Error listing products route: {:?}", e);
            HttpResponse::NotFound().json(e.to_string())
        }
    }
}

#[tracing::instrument(skip(update_product), fields(log_name))]
pub(crate) async fn update_product(update_product: web::Json<UpdateProduct>) -> impl Responder {
    let log_name = update_product.0.name.clone();
    match UpdateProduct::update_product(update_product.into_inner()).await {
        Ok(_) => {
            info!("Updated product route: {:?}", log_name);
            HttpResponse::Created().json("Product update successful!".to_string())
        }
        Err(e) => {
            error!("Error updating product route: {:?}", log_name);
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

#[tracing::instrument(skip(product_name), fields(log_name))]
pub(crate) async fn get_product(product_name: web::Json<ProductStruct>) -> impl Responder {
    let log_name = product_name.0.name.clone();
    match ProductStruct::get_product(product_name.into_inner()).await {
        Ok(e) => {
            info!("Get product route: {:?}", log_name);
            HttpResponse::Ok().json(e)
        }
        Err(e) => {
            error!("Error getting product route: {:?}", log_name);
            HttpResponse::NotFound().json(e.to_string())
        }
    }
}
