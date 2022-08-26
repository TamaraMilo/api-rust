use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum,Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(1))",
    enum_name = "role"
)]
pub enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "User")]
    User
}
impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: Role,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}






