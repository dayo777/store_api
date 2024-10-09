use crate::models::customer::{
    ChangeEmailStruct, ChangePasswordStruct, Customer, CustomerEmailStruct, NewCustomer,
    UpdateCustomerStruct,
};
use actix_web::{web, HttpResponse, Responder};

pub(crate) async fn create_new_customer(new_customer: web::Json<NewCustomer>) -> impl Responder {
    match NewCustomer::new_customer(new_customer.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("New Customer details saved successfully!"),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub(crate) async fn get_all_customers() -> impl Responder {
    match Customer::get_all_customers().await {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(_) => HttpResponse::BadRequest().body("Issue fetching all customers."),
    }
}

pub(crate) async fn delete_customer(
    customer_email: web::Json<CustomerEmailStruct>,
) -> impl Responder {
    match Customer::delete_customer(customer_email.into_inner().email).await {
        Ok(delete_msg) => HttpResponse::Ok().json(delete_msg.as_str()),
        Err(_) => HttpResponse::NotFound().json("Customer does not exist."),
    }
}

pub(crate) async fn get_customer(customer_email: web::Json<CustomerEmailStruct>) -> impl Responder {
    match CustomerEmailStruct::get_customer(customer_email.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub(crate) async fn change_email(new_email: web::Json<ChangeEmailStruct>) -> impl Responder {
    match ChangeEmailStruct::update_email(new_email.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Email successfully updated!"),
        Err(e) => HttpResponse::Unauthorized().json(e.to_string()),
    }
}

pub(crate) async fn change_password(
    password_struct: web::Json<ChangePasswordStruct>,
) -> impl Responder {
    match ChangePasswordStruct::change_password(password_struct.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Password successfully updated!"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub(crate) async fn update_customer(
    customer_update: web::Json<UpdateCustomerStruct>,
) -> impl Responder {
    match UpdateCustomerStruct::update_customer(customer_update.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Customer details successfully updated!"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to update customer."),
    }
}
