use crate::domain::utils;
use crate::error::OrderError;
use crate::models::order::{NewOrder, Order, OrderID, OrderStatus, OrderUpdateStatus};
use tracing::error;
use validator::Validate;

// import Customer structs
use customer::models::customer::CustomerEmailStruct;
use database::DB;

// TODO: Bug report, if i make two orders, and one goes because it's less than product inventory, while the other is more than product inventory...
// ...an order is not created but subtraction from product inventory occurs. All orders should either succeed together or fail togther.

impl NewOrder {
    // the whole idea is, this endpoint is called after the customer (on the frontend) is checking out their carts...
    // ... and payments has been validated, then an order is created.
    pub(crate) async fn new_order(new_order: NewOrder) -> Result<(), OrderError> {
        // perform Struct validation check
        let new_order = NewOrder {
            customer_email: new_order.customer_email.clone(),
            items: new_order.items.clone(),
            shipping_address: new_order.shipping_address.clone(),
        };
        if let Err(e) = new_order.validate() {
            error!("Error validating new order: {}", e);
            return Err(OrderError::ValidationError(e));
        }

        // retrieve Customer details
        let email: CustomerEmailStruct = CustomerEmailStruct {
            email: new_order.customer_email.clone(),
        };
        let customer_id = CustomerEmailStruct::get_customer(email).await?;

        // populate OrderItems field
        let order_items = utils::construct_order_items(new_order.items).await?;
        println!("{:?}", order_items.clone());

        // deduct quantities from Product stock inventory
        utils::deduct_batch_product(order_items.clone()).await?;

        // populate other items
        let order_date = utils::get_datetime();
        let total_amount = utils::calculate_total_price(order_items.clone());
        let status = OrderStatus::Confirmed;

        let final_order = Order {
            id: None,
            customer_id,
            customer_email: new_order.customer_email,
            items: order_items,
            status,
            total_amount,
            shipping_address: new_order.shipping_address,
            order_date,
            updated_at: None,
        };

        let _: Option<Order> = DB.create("order").content(final_order).await?;
        Ok(())
    }
}

impl Order {
    pub(crate) async fn list_orders() -> Result<Vec<Order>, OrderError> {
        let orders: Vec<Order> = DB.select("order").await?;
        Ok(orders)
    }
}

impl OrderID {
    // utilizing Customer-email as ID
    pub async fn get_order_id(order_id: OrderID) -> Result<Order, OrderError> {
        let order_id = order_id.customer_email.to_lowercase();
        let validation_check1 = OrderID {
            customer_email: order_id.clone(),
        };
        if let Err(e) = validation_check1.validate() {
            error!("Error retrieving order, validation failed: {}", e);
            return Err(OrderError::ValidationError(e));
        }

        let mut response = DB
            .query("SELECT * FROM order WHERE customer_email = $ce")
            .bind(("ce", order_id))
            .await?;

        let result: Option<Order> = response.take(0)?;
        match result {
            Some(order) => Ok(order),
            None => Err(OrderError::InvalidOrder),
        }
    }
}

impl OrderUpdateStatus {
    pub async fn update_status(order_update: OrderUpdateStatus) -> Result<String, OrderError> {
        let validation_check = OrderUpdateStatus {
            customer_email: order_update.customer_email.clone(),
            status: order_update.status.clone(),
        };
        if let Err(e) = validation_check.validate() {
            error!("Error updating customer status, validation failed: {}", e);
            return Err(OrderError::ValidationError(e));
        }

        let updated_at = utils::get_datetime();
        let mut response = DB
            .query("UPDATE order SET status = $status, updated_at = $updated_at WHERE customer_email = $email")
            .bind(("status", order_update.status))
            .bind(("updated_at", updated_at))
            .bind(("email", order_update.customer_email))
            .await?;

        let result: Option<Order> = response.take(0)?;
        match result {
            Some(value) => Ok(format!(
                "Hi {}, your order status has been updated to {}!",
                value.customer_id.name, value.status
            )),
            None => Err(OrderError::InvalidOrder),
        }
    }
}
