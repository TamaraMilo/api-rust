use crate::auth::dto::UserIdetificationDTO;
use crate::{
    context::AppState,
    errors::Errors,
    user::{
        dto::UserCreateDTO,
        user_repository::{create_user, user_exist},
    },
};
use actix_web::web;
use entity::user::Role;
use pwhash::bcrypt;
use validator::Validate;

use super::dto::{LoginRequest, UserData, UserClaims};

pub fn user_verify(id: String, user_claims: UserClaims) -> bool {
    if id != user_claims.id {
        if user_claims.role != Role::Admin {
            return false;
        }
    }
    true
}

pub async fn singin_user(
    data: web::Data<AppState>,
    user: web::Json<LoginRequest>,
) -> Result<UserData, Errors> {
    let user_db = user_exist(
        &data.conn,
        UserIdetificationDTO {
            email: user.identifier.to_string(),
            username: user.identifier.to_string(),
        },
    )
    .await
    .map_err(|_| return Errors::DatabaseError)?;

    let user_db = match user_db {
        Some(user) => user,
        None => return Err(Errors::NoUser),
    };
    if !bcrypt::verify(user.password.to_string(), &user_db.password) {
        return Err(Errors::IncorrectUserPassword);
    };
    Ok(UserData {
        user_id: user_db.user_id,
        username: user_db.username,
        email: user_db.email,
        role: user_db.role,
    })
}
pub async fn singup_user(
    data: web::Data<AppState>,
    user: web::Json<UserCreateDTO>,
) -> Result<UserData, Errors> {
    user.validate()
        .map_err(|_| return Errors::PassAndUsernameError)?;

    let user_exist = user_exist(
        &data.conn,
        UserIdetificationDTO {
            email: user.email.to_string(),
            username: user.username.to_string(),
        },
    )
    .await
    .map_err(|_| return Errors::DatabaseError)?;
    if user_exist.is_some() {
        return Err(Errors::PassAndUsernameError);
    }
    let new_user = create_user(&data.conn, user.0)
        .await
        .map_err(|_| return Errors::DatabaseError)?;

    Ok(new_user)
}
