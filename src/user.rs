
use pwhash::bcrypt;
use migration::{Condition, DbErr};
use sea_orm::{DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, IntoActiveModel};
use sea_orm::ActiveModelTrait;
use crate::auth::{SingInRequest, UserIdetifier};
use crate::responses::{ LoginResponse};
use entity::user::{Model};
use entity::user::Role;
use entity::user::Entity as User;

pub async fn add_user(db: &DatabaseConnection, user: SingInRequest) ->Result<LoginResponse, DbErr>{


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
        Ok(user) => return Ok(LoginResponse::new(user)),
        Err(e) => return Err(e)
    }

    
}
pub async fn user_exist(db: &DatabaseConnection, user: UserIdetifier) -> Result<Option<Model>, DbErr> 
{
    let condition = Condition::any()
                .add(entity::user::Column::Username.eq(user.username.to_string()))
                .add(entity::user::Column::Email.eq(user.email.to_string()));


    match User::find().filter(condition).one(db).await {
            Ok(result)=> return Ok(result),
            Err(e)=> return Err(e)
        };


} 
