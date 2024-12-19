use crate::models::customer::{
    ChangeEmailStruct, ChangePasswordStruct, Customer, CustomerEmailStruct, NewCustomer,
    UpdateCustomerStruct,
};
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};

#[tracing::instrument(skip(new_customer), fields(log_name))]
pub(crate) async fn create_new_customer(new_customer: web::Json<NewCustomer>) -> impl Responder {
    let log_name = new_customer.0.name.clone();
    match NewCustomer::new_customer(new_customer.into_inner()).await {
        Ok(_) => {
            info!("Creating new-customer route: {}", log_name);
            HttpResponse::Created().json("New Customer details saved successfully!")
        }
        Err(e) => {
            error!("Error creating new-customer route: {}", log_name);
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
}

#[tracing::instrument()]
pub(crate) async fn get_all_customers() -> impl Responder {
    match Customer::get_all_customers().await {
        Ok(customer) => {
            info!("List customers route: {}", customer.len());
            HttpResponse::Ok().json(customer)
        }
        Err(_) => {
            error!("Error getting all-customers!");
            HttpResponse::BadRequest().body("Issue fetching all customers.")
        }
    }
}

#[tracing::instrument(skip(customer_email), fields(log_name))]
pub(crate) async fn delete_customer(
    customer_email: web::Json<CustomerEmailStruct>,
) -> impl Responder {
    // might not want to log this due to privacy concerns, but this is only for testing purposes
    let log_name = customer_email.0.email.clone();
    match Customer::delete_customer(customer_email.into_inner().email).await {
        Ok(delete_msg) => {
            info!("Deleted customer route: {}", log_name);
            HttpResponse::Ok().json(delete_msg.as_str())
        }
        Err(_) => {
            error!("Error deleting customer route: {}", log_name);
            HttpResponse::NotFound().json("Customer does not exist.")
        }
    }
}

#[tracing::instrument(skip(customer_email), fields(log_name))]
pub(crate) async fn get_customer(customer_email: web::Json<CustomerEmailStruct>) -> impl Responder {
    let log_name = customer_email.0.email.clone();
    match CustomerEmailStruct::get_customer(customer_email.into_inner()).await {
        Ok(response) => {
            info!("Get customer route: {}", log_name);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            error!("Error getting customer route: {}", log_name);
            HttpResponse::NotFound().json(e.to_string())
        }
    }
}

#[tracing::instrument(skip(new_email), fields(log_name))]
pub(crate) async fn change_email(new_email: web::Json<ChangeEmailStruct>) -> impl Responder {
    let log_name = new_email.0.email.clone();
    let log_name2 = new_email.0.new_email.clone();
    match ChangeEmailStruct::update_email(new_email.into_inner()).await {
        Ok(_) => {
            info!(
                "Updated customer-email route: from '{}' to '{}'",
                log_name, log_name2
            );
            HttpResponse::Ok().json("Email successfully updated!")
        }
        Err(e) => HttpResponse::Unauthorized().json(e.to_string()),
    }
}

#[tracing::instrument(skip(password_struct), fields(log_name))]
pub(crate) async fn change_password(
    password_struct: web::Json<ChangePasswordStruct>,
) -> impl Responder {
    let log_name = password_struct.0.email.clone();
    match ChangePasswordStruct::change_password(password_struct.into_inner()).await {
        Ok(_) => {
            info!("Change password route for user: {}", log_name);
            HttpResponse::Ok().json("Password successfully updated!")
        }
        Err(e) => {
            error!("Error updating password route: {}", log_name);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

#[tracing::instrument(skip(customer_update), fields(log_name))]
pub(crate) async fn update_customer(
    customer_update: web::Json<UpdateCustomerStruct>,
) -> impl Responder {
    let log_name = customer_update.0.email.clone();
    match UpdateCustomerStruct::update_customer(customer_update.into_inner()).await {
        Ok(_) => {
            info!("Updating customer route: {}", log_name);
            HttpResponse::Ok().json("Customer details successfully updated!")
        }
        Err(_) => {
            error!("Error updating customer route: {}", log_name);
            HttpResponse::InternalServerError().json("Unable to update customer.")
        }
    }
}
