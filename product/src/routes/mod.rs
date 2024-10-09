use actix_web::web;

mod product;

// add routes below
pub fn product_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(product::create_product))
            .route(web::get().to(product::get_product)),
    );
    cfg.service(web::resource("/list/").route(web::get().to(product::list_product)));
    cfg.service(web::resource("/update/").route(web::patch().to(product::update_product)));
}
