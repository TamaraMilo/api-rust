use entity::user::{Model, Role};
use migration::{DbErr, Condition};
use pwhash::bcrypt;
use sea_orm::{DatabaseConnection, IntoActiveModel, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use entity::user::Entity as User;
use crate::{ auth::dto::{UserData, UserIdetificationDTO}};

use super::dto::{UserCreateDTO};



pub async fn create_user(db: &DatabaseConnection,user: UserCreateDTO) ->Result<UserData, DbErr> 
{

    let hash_password =match  bcrypt::hash(user.password.to_string()) {
        Ok(r)=>r,
        Err(_)=> return Err(DbErr::Type("hash error".to_string()))
    };

    let user_id = uuid::Uuid::new_v4().to_string();
    let account = Model{
        user_id: user_id,
        username: user.username,
        password: hash_password,
        email: user.email,
        role: Role::default()
    }.into_active_model();


    match account.insert(db).await {
        Ok(user) => return Ok(UserData::new(user)),
        Err(e) => return Err(e)
    }

}
pub async fn user_exist(db: &DatabaseConnection, user: UserIdetificationDTO) -> Result<Option<Model>, DbErr> 
{
    let condition = Condition::any()
                .add(entity::user::Column::Username.eq(user.username.to_string()))
                .add(entity::user::Column::Email.eq(user.email.to_string()));

    User::find().filter(condition).one(db).await.map_err(|_| return DbErr::RecordNotFound("No user".to_string()))
} 
pub async fn update_user(db : &DatabaseConnection, user_id: String) -> Result<Model, DbErr>
{
    let user = User::find_by_id(user_id).one(db).await?;
    let mut user = match user {
        Some(user) =>user,
        None => return Err(DbErr::RecordNotFound("No user".to_string()))
    }.into_active_model();
    user.role = Set(Role::Admin);
    user.update(db).await

}
