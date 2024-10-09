use actix_web::web;

mod category;

// add routes here
pub fn category_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(category::create_new_category))
            .route(web::get().to(category::get_category)),
    );
    cfg.service(web::resource("/list/").route(web::get().to(category::list_categories)));
    cfg.service(web::resource("/delete/").route(web::delete().to(category::delete_category)));
    cfg.service(
        web::resource("/update-desc/")
            .route(web::patch().to(category::update_category_description)),
    );
}
