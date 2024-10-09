use crate::models::category::{Category, CategoryStruct, NewCategory, UpdateCategoryDesc};
use actix_web::{web, HttpResponse, Responder};

pub(crate) async fn create_new_category(new_category: web::Json<NewCategory>) -> impl Responder {
    match NewCategory::new_category(new_category.into_inner()).await {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub(crate) async fn get_category(category_uri: web::Json<CategoryStruct>) -> impl Responder {
    match CategoryStruct::get_category(category_uri.into_inner()).await {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub(crate) async fn list_categories() -> impl Responder {
    match Category::list_categories().await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

pub(crate) async fn delete_category(delete_category: web::Json<CategoryStruct>) -> impl Responder {
    match CategoryStruct::delete_category(delete_category.into_inner()).await {
        Ok(msg) => HttpResponse::Ok().json(msg),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

pub(crate) async fn update_category_description(
    update_desc: web::Json<UpdateCategoryDesc>,
) -> impl Responder {
    match UpdateCategoryDesc::update_category_description(update_desc.into_inner()).await {
        Ok(msg) => HttpResponse::Ok().json(msg),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}
