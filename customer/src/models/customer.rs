use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use validator::Validate;

/// Customer Validation
/// Password => 4 - 500
/// name => 3 - 20
/// age => 18 - 90

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CustomerType {
    Regular,
    Vip,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct NewCustomer {
    pub(crate) customer_type: CustomerType,
    #[validate(length(min = 3, max = 20))]
    pub(crate) name: String,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 4, max = 500))]
    pub(crate) password: String,
    #[validate(range(min = 18, max = 90))]
    pub(crate) age: u8,
    pub(crate) phone: String,
    pub(crate) address: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Customer {
    pub id: Option<RecordId>,
    pub customer_type: CustomerType,
    #[validate(length(min = 3, max = 20))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4, max = 500))]
    pub password: String,
    #[validate(range(min = 18, max = 90))]
    pub age: u8,
    pub phone: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// struct to update the Customer details
#[derive(Clone, Deserialize, Validate)]
pub(crate) struct UpdateCustomerStruct {
    pub(crate) customer_type: Option<CustomerType>,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 3, max = 20))]
    pub(crate) name: Option<String>,
    pub(crate) phone: Option<String>,
    pub(crate) address: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CustomerEmailStruct {
    #[validate(email)]
    pub email: String,
}

#[derive(Validate, Deserialize)]
pub(crate) struct ChangePasswordStruct {
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 4, max = 500))]
    pub(crate) password: String,
    #[validate(length(min = 4, max = 500))]
    pub(crate) new_password: String,
}

#[derive(Deserialize, Clone, Validate)]
pub(crate) struct ChangeEmailStruct {
    #[validate(email)]
    pub(crate) email: String,
    #[validate(email)]
    pub(crate) new_email: String,
}

// impl std::str::FromStr for CustomerType {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "regular" => Ok(Self::Regular),
//             "vip" => Ok(Self::VIP),
//             _ => Err(format!("Unable to convert to Customer-type: '{}'", s))
//         }
//     }
// }

// impl std::fmt::Display for CustomerType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Regular => write!(f, "Regular"),
//             Self::VIP => write!(f, "VIP"),
//         }
//     }
// }
