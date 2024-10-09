use actix_web::web;

pub mod customer;

// add routes below
pub fn customer_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(customer::create_new_customer))
            .route(web::get().to(customer::get_customer)),
    );
    cfg.service(web::resource("/list/").route(web::get().to(customer::get_all_customers)));
    cfg.service(web::resource("/delete/").route(web::delete().to(customer::delete_customer)));
    cfg.service(
        web::resource("/change-password/").route(web::patch().to(customer::change_password)),
    );
    cfg.service(web::resource("/update/").route(web::patch().to(customer::update_customer)));
    cfg.service(web::resource("/change-email/").route(web::patch().to(customer::change_email)));
}
