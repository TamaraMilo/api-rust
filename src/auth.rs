use core::fmt;

use actix_jwt_auth_middleware::FromRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone)]
pub enum Role {
    Admin,
    BaseUser,
}
impl fmt::Display for Role {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            Role::Admin => write!(f,"Admin"),
            Role::BaseUser=> write!(f,"BaseUser")
        }
    }
}

#[derive(Serialize,Deserialize,Clone,FromRequest)]
pub struct UserClaims  {
    pub id: String,
    pub role: Role
}







