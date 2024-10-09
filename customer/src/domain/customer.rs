use super::utils;
use crate::error::CustomerError;
use crate::models::customer::{
    ChangeEmailStruct, ChangePasswordStruct, Customer, CustomerEmailStruct, NewCustomer,
    UpdateCustomerStruct,
};
use database::DB;
use validator::Validate;

impl NewCustomer {
    pub(crate) async fn new_customer(new_customer: NewCustomer) -> Result<(), CustomerError> {
        // check if email already exist
        let email_exists =
            utils::check_if_email_exists(new_customer.email.clone().to_lowercase()).await;
        if email_exists {
            return Err(CustomerError::DuplicateEmailAddress);
        }

        // check if name already exist
        let name_exists =
            utils::check_if_name_exists(new_customer.name.clone().to_lowercase()).await;
        if name_exists {
            return Err(CustomerError::DuplicateEmailAddress);
        }

        let created_at = utils::get_datetime();
        let name = new_customer.name.to_lowercase();
        let email = new_customer.email.to_lowercase();
        let password = utils::encrypt_password(new_customer.password)?;

        let new_customer = Customer {
            id: None,
            customer_type: new_customer.customer_type,
            name,
            email,
            password,
            age: new_customer.age,
            phone: new_customer.phone,
            address: new_customer.address,
            created_at,
            updated_at: None,
        };

        // Validate details
        if let Err(e) = new_customer.validate() {
            return Err(CustomerError::ValidationFailed(e));
        }

        let _: Option<Customer> = DB.create("customer").content(new_customer).await?;

        Ok(())
    }
}

impl Customer {
    pub(crate) async fn get_all_customers() -> Result<Vec<Customer>, CustomerError> {
        let all_customer: Vec<Customer> = DB.select("customer").await?;
        Ok(all_customer)
    }

    pub(crate) async fn delete_customer(email: String) -> Result<String, CustomerError> {
        let email_check = utils::check_if_email_exists(email.clone()).await;
        if !email_check {
            return Err(CustomerError::CustomerDoesNotExist);
        }

        let _response = DB
            .query("DELETE customer WHERE email = $delete_email")
            .bind(("delete_email", email.to_lowercase()))
            .await?;

        Ok(format!("User account '{}', successfully deleted!", email))
    }
}

impl ChangePasswordStruct {
    pub(crate) async fn change_password(
        password_change: ChangePasswordStruct,
    ) -> Result<(), CustomerError> {
        // perform validation check using the Validator crate
        let validation_check_1 = ChangePasswordStruct {
            email: password_change.email.clone(),
            password: password_change.password.clone(),
            new_password: password_change.new_password.clone(),
        };
        if let Err(e) = validation_check_1.validate() {
            return Err(CustomerError::ValidationFailed(e));
        }

        // basic check to confirm different inputs for old & new password
        let validation_check_2 = utils::confirm_old_new_password_are_different(
            password_change.password.clone(),
            password_change.new_password.clone(),
        );
        if validation_check_2 {
            return Err(CustomerError::PasswordMatchError);
        }

        // confirm email & password pair exist, before updating password
        if let Err(e) = utils::is_valid_user(
            password_change.email.clone(),
            password_change.password.clone(),
        )
        .await
        {
            Err(e)
        } else {
            let updated_at = utils::get_datetime();
            let new_password = utils::encrypt_password(password_change.new_password)?;
            let _response = DB
                .query(
                    "
                    UPDATE customer
                    SET password = $new_password, updated_at = $updated_at
                    WHERE email = $email
                    ",
                )
                .bind(("new_password", new_password))
                .bind(("updated_at", updated_at))
                .bind(("email", password_change.email))
                .await?;
            Ok(())
        }
    }
}

// Age should not be updated
impl UpdateCustomerStruct {
    pub(crate) async fn update_customer(
        update_customer: UpdateCustomerStruct,
    ) -> Result<(), CustomerError> {
        let email_check = utils::check_if_email_exists(update_customer.email.clone()).await;
        if !email_check {
            return Err(CustomerError::CustomerDoesNotExist);
        }

        let updated_at = utils::get_datetime();

        // update the name if supplied
        if let Some(update_name) = update_customer.name {
            let _ = DB
                .query("UPDATE customer SET name = $name, updated_at = $updated_at WHERE email = $email")
                .bind(("name", update_name))
                .bind(("updated_at", updated_at))
                .bind(("email", update_customer.email.clone()))
                .await?;
        }

        // update the address if supplied
        if let Some(update_address) = update_customer.address {
            let _ = DB
                .query("UPDATE customer SET address = $address, updated_at = $updated_at WHERE email = $email")
                .bind(("address", update_address))
                .bind(("updated_at", updated_at))
                .bind(("email", update_customer.email.clone()))
                .await?;
        }

        // update the phone number if supplied
        if let Some(update_phone) = update_customer.phone {
            let _ = DB
                .query("UPDATE customer SET phone = $phone, updated_at = $updated_at WHERE email = $email")
                .bind(("phone", update_phone))
                .bind(("updated_at", updated_at))
                .bind(("email", update_customer.email.clone()))
                .await?;
        }

        if let Some(customer_type) = update_customer.customer_type {
            let _ = DB
                .query("UPDATE customer SET customer_type = $customer_type, updated_at = $updated_at WHERE email = $email")
                .bind(("customer_type", customer_type))
                .bind(("updated_at", updated_at))
                .bind(("email", update_customer.email))
                .await?;
        }

        Ok(())
    }
}

impl CustomerEmailStruct {
    pub async fn get_customer(
        customer_email: CustomerEmailStruct,
    ) -> Result<Customer, CustomerError> {
        let email = customer_email.email.to_lowercase();
        let customer_email = CustomerEmailStruct {
            email: email.clone(),
        };
        if let Err(e) = customer_email.validate() {
            return Err(CustomerError::ValidationFailed(e));
        }

        let email_check = utils::check_if_email_exists(email.clone()).await;
        if !email_check {
            return Err(CustomerError::CustomerDoesNotExist);
        }

        let mut response = DB
            .query("SELECT * FROM customer WHERE email = $email")
            .bind(("email", email.to_lowercase()))
            .await
            .map_err(|_| CustomerError::CustomerDoesNotExist)?;

        let customer_detail: Option<Customer> = response.take(0)?;

        Ok(customer_detail.unwrap())
    }
}

impl ChangeEmailStruct {
    pub(crate) async fn update_email(email_struct: ChangeEmailStruct) -> Result<(), CustomerError> {
        // struct validation checks
        let email_check = ChangeEmailStruct {
            email: email_struct.email.clone(),
            new_email: email_struct.email.clone(),
        };
        if let Err(e) = email_check.validate() {
            return Err(CustomerError::ValidationFailed(e));
        }

        // basic checks
        if utils::confirm_old_new_password_are_different(
            email_struct.email.clone(),
            email_struct.new_email.clone(),
        ) {
            return Err(CustomerError::SameEmailError);
        }

        // check if email exist
        let email_check = utils::check_if_email_exists(email_struct.email.to_lowercase()).await;
        if !email_check {
            return Err(CustomerError::CustomerDoesNotExist);
        }

        // check if new_email already exist in the DB
        let new_email_check =
            utils::check_if_email_exists(email_struct.new_email.to_lowercase()).await;
        if new_email_check {
            return Err(CustomerError::SameEmailError);
        }

        let datetime = utils::get_datetime();
        let _response = DB
            .query("UPDATE customer SET email = $new_email WHERE email = $email")
            .query("UPDATE customer SET updated_at = $updated_at WHERE email = $new_email")
            .bind(("new_email", email_struct.new_email))
            .bind(("email", email_struct.email))
            .bind(("updated_at", datetime))
            .await?;

        Ok(())
    }
}
