
use actix_jwt_auth_middleware::FromRequest;
use entity::user::{Role, Model};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Clone,FromRequest)]
pub struct UserClaims  {
    pub id: String,
    pub role: Role
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserIdetificationDTO{
    pub email: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct UserData {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: Role,

}
impl UserData {
    pub fn new(user: Model) -> Self {
        Self{
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    } 
}
#[derive(Serialize,Deserialize,Clone)]
pub struct LoginResponse
{
    pub user: UserData,
    pub token: String,
}




