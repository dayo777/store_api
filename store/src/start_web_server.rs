use actix_web::{web, App, HttpServer};
use std::error::Error;

// import routes configuration here 👇
use category::routes::category_config;
use customer::routes::customer_config;
use orders::routes::order_config;
use product::routes::product_config;

// TODO: handle the error properly
pub(crate) async fn start_web_server() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("Unable to get web server environment variables.");
    let hostname = std::env::var("WEB_HOSTNAME").expect("Unable to retrieve hostname.");

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1/customer").configure(customer_config))
            .service(web::scope("/api/v1/category").configure(category_config))
            .service(web::scope("/api/v1/product").configure(product_config))
            .service(web::scope("/api/v1/order").configure(order_config))
    })
    .bind(hostname.as_str())?
    .run()
    .await?;

    Ok(())
}
