use super::utils;
use crate::error::CategoryError;
use crate::models::category::{Category, CategoryStruct, NewCategory, UpdateCategoryDesc};
use database::DB;
use validator::Validate;

// convert to lowercase.
impl NewCategory {
    pub(crate) async fn new_category(new_category: NewCategory) -> Result<String, CategoryError> {
        // confirm duplicate category does no exist
        let category_check = utils::check_if_category_exist(new_category.name.clone()).await;
        if category_check {
            return Err(CategoryError::DuplicateCategory);
        }

        let created_at = utils::get_datetime();

        let cat = Category {
            id: None,
            name: new_category.name.to_lowercase(),
            description: new_category.description,
            created_at,
            updated_at: None,
        };

        if let Err(e) = cat.validate() {
            return Err(CategoryError::ValidationFailed(e));
        }

        let _: Option<Category> = DB.create("category").content(cat).await?;

        Ok(format!("Category '{}' created", new_category.name))
    }
}

impl Category {
    pub(crate) async fn list_categories() -> Result<Vec<Category>, CategoryError> {
        let all_category: Vec<Category> = DB.select("category").await?;
        Ok(all_category)
    }
}

// to update the Category description
impl UpdateCategoryDesc {
    pub(crate) async fn update_category_description(
        update_category: UpdateCategoryDesc,
    ) -> Result<String, CategoryError> {
        // check if object exist, & fail if not
        let category_check =
            utils::check_if_category_exist(update_category.name.to_lowercase()).await;
        if !category_check {
            return Err(CategoryError::NotFound);
        }

        let check_validation = UpdateCategoryDesc {
            name: update_category.name.clone(),
            new_description: update_category.new_description.clone(),
        };

        if let Err(e) = check_validation.validate() {
            return Err(CategoryError::ValidationFailed(e));
        }

        let updated_at = utils::get_datetime();
        let _response = DB
            .query("UPDATE category SET description = $desc, updated_at = $updated_at WHERE name = $name")
            .bind(("desc", update_category.new_description))
            .bind(("name", update_category.name.to_lowercase()))
            .bind(("updated_at", updated_at))
            .await?;

        Ok(format!(
            "Description for category '{}' successfully updated!",
            update_category.name
        ))
    }
}

impl CategoryStruct {
    pub(crate) async fn get_category(
        category_struct: CategoryStruct,
    ) -> Result<Category, CategoryError> {
        // // check if object exist, & fail if not
        // let category_check = utils::check_if_category_exist(category_struct.name.to_lowercase()).await;
        // if !category_check {
        //     return Err(CategoryError::NotFound);
        // }

        let mut response = DB
            .query("SELECT * FROM category WHERE name = $name")
            .bind(("name", category_struct.name.to_lowercase()))
            .await?;

        let response: Option<Category> = response.take(0)?;

        match response {
            Some(resp) => Ok(resp),
            None => Err(CategoryError::NotFound),
        }
    }

    pub(crate) async fn delete_category(
        category_struct: CategoryStruct,
    ) -> Result<String, CategoryError> {
        // check if object exist, & fail if not
        let category_check =
            utils::check_if_category_exist(category_struct.name.to_lowercase()).await;
        if !category_check {
            return Err(CategoryError::NotFound);
        }

        let _response = DB
            .query("DELETE category WHERE name = $name")
            .bind(("name", category_struct.name.to_lowercase()))
            .await?;
        Ok(format!(
            "Category `{}` successfully deleted!",
            category_struct.name
        ))
    }
}
