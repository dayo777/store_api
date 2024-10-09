use super::utils;
use crate::error::ProductError;
use crate::models::product::{NewProduct, Product, ProductStruct, UpdateProduct};
use database::DB;
use validator::Validate;

use category::check_if_category_exist;

impl NewProduct {
    pub(crate) async fn create_new_product(
        new_product: NewProduct,
    ) -> Result<String, ProductError> {
        // basic validation check
        let product_basic_check = NewProduct {
            name: new_product.name.clone(),
            description: new_product.description.clone(),
            price: new_product.price,
            category: new_product.category.clone(),
            stock_quantity: new_product.stock_quantity,
        };
        if let Err(e) = product_basic_check.validate() {
            return Err(ProductError::ValidationError(e));
        }

        // confirm product Name is not a duplicate
        let name_exists = utils::check_if_product_exist(new_product.name.to_lowercase()).await;
        if name_exists {
            return Err(ProductError::DuplicateProductName);
        }

        // confirm that category name exists
        let category_exists = check_if_category_exist(new_product.category.to_lowercase()).await;
        if !category_exists {
            return Err(ProductError::InvalidCategory);
        }

        let created_at = utils::get_datetime();
        let final_product = Product {
            id: None,
            name: new_product.name.to_lowercase(),
            description: new_product.description,
            price: new_product.price,
            category: new_product.category,
            stock_quantity: new_product.stock_quantity,
            created_at,
            updated_at: None,
        };

        let _: Option<Product> = DB.create("product").content(final_product).await?;
        Ok(format!(
            "Product '{}' with stock quantity '{}' created successfully!",
            new_product.name, new_product.stock_quantity
        ))
    }
}

impl Product {
    pub(crate) async fn list_products() -> Result<Vec<Product>, ProductError> {
        let products: Vec<Product> = DB.select("product").await?;
        Ok(products)
    }
}

impl UpdateProduct {
    pub(crate) async fn update_product(update_product: UpdateProduct) -> Result<(), ProductError> {
        let product_check = utils::check_if_product_exist(update_product.name.to_lowercase()).await;
        if !product_check {
            return Err(ProductError::ProductDoesNotExist);
        }

        let updated_at = utils::get_datetime();

        // update the description if supplied
        if let Some(update_description) = update_product.description {
            let _ = DB
                .query("UPDATE product SET description = $desc, updated_at = $updated_at WHERE name = $name")
                .bind(("desc", update_description))
                .bind(("updated_at", updated_at))
                .bind(("name", update_product.name.to_lowercase()))
                .await?;
        }

        // update price if supplied
        if let Some(update_price) = update_product.price {
            let _ = DB
                .query("UPDATE product SET price = $price, updated_at = $updated_at WHERE name = $name")
                .bind(("price", update_price))
                .bind(("updated_at", updated_at))
                .bind(("name", update_product.name.to_lowercase()))
                .await?;
        }

        // increase the quantity if supplied
        if let Some(updated_stock_quantity) = update_product.stock_quantity {
            let _ = DB
                .query("UPDATE product SET stock_quantity += $qty, updated_at = $updated_at WHERE name = $name")
                .bind(("qty", updated_stock_quantity))
                .bind(("updated_at", updated_at))
                .bind(("name", update_product.name.to_lowercase()))
                .await?;
        }

        Ok(())
    }
}

impl ProductStruct {
    pub(crate) async fn get_product(name: ProductStruct) -> Result<Product, ProductError> {
        let product_name = name.name.to_lowercase();

        let mut response = DB
            .query("SELECT * FROM product WHERE name = $name")
            .bind(("name", product_name))
            .await
            .map_err(|_| ProductError::ProductDoesNotExist)?;

        let product_details: Option<Product> = response.take(0)?;
        // Ok(product_details.unwrap())
        match product_details {
            Some(product_details) => Ok(product_details),
            None => Err(ProductError::ProductDoesNotExist),
        }
    }
}
