mod order;

use actix_web::web;

pub fn order_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(order::create_new_order))
            .route(web::get().to(order::get_order)),
    );
    cfg.service(web::resource("/list/").route(web::get().to(order::list_orders)));
    cfg.service(
        web::resource("/update-status/").route(web::patch().to(order::order_update_status)),
    );
}
