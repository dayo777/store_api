use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

// ValidationChecks
// name => min of 5 characters
// description => min of 6 characters

#[derive(Deserialize, Validate)]
pub(crate) struct NewCategory {
    #[validate(length(min = 5))]
    pub(crate) name: String,
    #[validate(length(min = 6))]
    pub(crate) description: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Category {
    pub id: Option<Thing>,
    #[validate(length(min = 5))]
    pub name: String,
    #[validate(length(min = 6))]
    pub(crate) description: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub(crate) struct CategoryStruct {
    pub(crate) name: String,
}

#[derive(Deserialize, Validate)]
pub(crate) struct UpdateCategoryDesc {
    pub(crate) name: String,
    #[validate(length(min = 6))]
    pub(crate) new_description: String,
}
