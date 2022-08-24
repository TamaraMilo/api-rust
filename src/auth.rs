
use actix_jwt_auth_middleware::FromRequest;
use entity::user::Role;
use serde::{Deserialize, Serialize};




#[derive(Serialize,Deserialize,Clone,FromRequest)]
pub struct UserClaims  {
    pub id: String,
    pub role: Role
}







