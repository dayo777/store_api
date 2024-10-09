use chrono::{DateTime, Utc};
use customer::models::customer::Customer;
use product::models::product::Product;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use validator::Validate;

// Order for creating & persisting a new Order to the database
#[derive(Deserialize, Validate, Clone)]
pub struct NewOrder {
    #[validate(email)]
    pub customer_email: String,
    #[validate(length(min = 1))]
    pub items: Vec<CreateOrderItem>,
    #[validate(length(min = 5, max = 150))]
    pub shipping_address: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Order {
    pub(crate) id: Option<RecordId>,
    pub(crate) customer_id: Customer,
    pub(crate) customer_email: String,
    pub(crate) items: Vec<OrderItem>,
    pub(crate) status: OrderStatus,
    pub(crate) total_amount: f32,
    pub(crate) shipping_address: String,
    pub(crate) order_date: DateTime<Utc>,
    pub(crate) updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Validate)]
pub(crate) struct OrderID {
    #[validate(email)]
    pub(crate) customer_email: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum OrderStatus {
    Confirmed, // Payment has been validated & Order placed
    Failed,    // Order processing failed, no payment or some internal error
    Pending,   // Order status is unknown
    InTransit, // Order is in transit to Customer address
    Delivered, // Customer has received order
}

#[derive(Deserialize, Clone, Serialize, Validate, Debug)]
pub(crate) struct OrderItem {
    pub(crate) product: Product,
    #[validate(range(min = 1))]
    pub(crate) quantity: u64,
}

#[derive(Deserialize, Clone, Serialize, Validate)]
pub(crate) struct CreateOrderItem {
    pub(crate) product_name: String,
    #[validate(range(min = 1))]
    pub(crate) quantity: u64,
}

#[derive(Deserialize, Validate)]
pub(crate) struct OrderUpdateStatus {
    #[validate(email)]
    pub(crate) customer_email: String,
    pub(crate) status: OrderStatus,
}

impl std::str::FromStr for OrderStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "confirmed" => Ok(Self::Confirmed),
            "failed" => Ok(Self::Failed),
            "pending" => Ok(Self::Pending),
            "intransit" => Ok(Self::InTransit),
            "delivered" => Ok(Self::Delivered),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Confirmed => write!(f, "confirmed"),
            Self::Delivered => write!(f, "delivered"),
            Self::Failed => write!(f, "failed"),
            Self::InTransit => write!(f, "intransit"),
            Self::Pending => write!(f, "pending"),
        }
    }
}
