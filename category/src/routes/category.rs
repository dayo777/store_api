use crate::models::category::{Category, CategoryStruct, NewCategory, UpdateCategoryDesc};
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};

#[tracing::instrument(fields(log_name), skip(new_category))]
pub(crate) async fn create_new_category(new_category: web::Json<NewCategory>) -> impl Responder {
    let log_name = new_category.0.name.clone();
    match NewCategory::new_category(new_category.into_inner()).await {
        Ok(category) => {
            // tracing::Span::current().record("log_name", log_name);
            info!("New category route: {}", log_name);
            HttpResponse::Created().json(category)
        }
        Err(e) => {
            // tracing::Span::current().record("log_name", log_name);
            error!("Error creating category: {}", log_name);
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

#[tracing::instrument(fields(log_name), skip(category_uri))]
pub(crate) async fn get_category(category_uri: web::Json<CategoryStruct>) -> impl Responder {
    let log_name = category_uri.0.name.clone();
    match CategoryStruct::get_category(category_uri.into_inner()).await {
        Ok(category) => {
            info!("Get category route: {}", log_name);
            HttpResponse::Ok().json(category) },
        Err(e) => {
            error!("Error getting category: {}", log_name);
            HttpResponse::NotFound().json(e.to_string())
        }
    }
}

#[tracing::instrument()]
pub(crate) async fn list_categories() -> impl Responder {
    match Category::list_categories().await {
        Ok(list) => {
            info!("List categories route: {}", list.len());
            HttpResponse::Ok().json(list)
        }
        Err(e) => {
            error!("Error listing categories route: {}", e);
            HttpResponse::NotFound().body(e.to_string())
        }
    }
}

#[tracing::instrument(skip(delete_category), fields(log_name))]
pub(crate) async fn delete_category(delete_category: web::Json<CategoryStruct>) -> impl Responder {
    let log_name = delete_category.0.name.clone();
    match CategoryStruct::delete_category(delete_category.into_inner()).await {
        Ok(msg) => {
            info!("Delete category route: {}", log_name);
            HttpResponse::Ok().json(msg) },
        Err(e) => {
            error!("Error deleting category route: {}", log_name);
            HttpResponse::NotFound().body(e.to_string())
        }
    }
}

#[tracing::instrument(fields(log_name), skip(update_desc))]
pub(crate) async fn update_category_description(
    update_desc: web::Json<UpdateCategoryDesc>,
) -> impl Responder {
    let log_name = update_desc.0.name.clone();
    match UpdateCategoryDesc::update_category_description(update_desc.into_inner()).await {
        Ok(msg) => {
            info!("Update category-desc route: {}", log_name);
            HttpResponse::Ok().json(msg)
        }
        Err(e) => {
            error!("Error updating category description route: {}", log_name);
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}
