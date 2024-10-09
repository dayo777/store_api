use crate::error::CustomerError;
use crate::models::customer::Customer;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use database::DB;

// return the DateTime in UTC
pub(crate) fn get_datetime() -> DateTime<Utc> {
    Utc::now()
}

// function to check if email exists in database
pub(crate) async fn check_if_email_exists(email_check: String) -> bool {
    let mut response = DB
        .query("SELECT * FROM customer WHERE email = $email_check")
        .bind(("email_check", email_check))
        .await
        .expect("Error checking duplicate email.");

    let response: Option<Customer> = response.take(0).expect("`Check if email exists` failed.");
    if let Some(_x) = response {
        return true;
    }
    false
}

pub(crate) async fn check_if_name_exists(name_check: String) -> bool {
    let mut response = DB
        .query("SELECT * FROM customer WHERE name = $name_check")
        .bind(("name_check", name_check))
        .await
        .expect("Error checking duplicate name.");

    let response: Option<Customer> = response.take(0).expect("`Check name exists` failed.");
    if response.is_some() {
        return true;
    }

    false
}

// confirm email & password are valid,
pub(crate) async fn is_valid_user(email: String, passwrd: String) -> Result<bool, CustomerError> {
    let mut response = DB
        .query("SELECT * FROM customer where email = $email")
        .bind(("email", email.clone()))
        .await?;

    // retrieve the hashed password from the DB
    let resp: Option<Customer> = response.take(0)?;
    let hashed_password = resp.unwrap().password.clone();

    if !verify(passwrd, hashed_password.as_str())? {
        Err(CustomerError::PasswordDoesNotExist)
    } else {
        Ok(true)
    }
}

pub(crate) fn confirm_old_new_password_are_different(
    old_password: String,
    new_password: String,
) -> bool {
    // old_password.as_str().eq(new_password.as_str())
    old_password.eq(new_password.as_str())
}

// encrypt password for DB storage
pub(crate) fn encrypt_password(password: String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// decrypt password
// pub(crate) fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
//     verify(password, &hash)
// }
