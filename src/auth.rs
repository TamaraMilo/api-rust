use core::fmt;
use std::str::FromStr;

use actix_jwt_auth_middleware::FromRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone,PartialEq)]
pub enum Role {
    Admin,
    User,
}
impl FromStr for Role {
    type Err = ();
    fn from_str(input: &str) -> Result<Role, Self::Err>{
        match input {
            "Admin" => Ok(Role::Admin),
            "User" => Ok(Role::User),
            _=> Err(()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            Role::Admin => write!(f,"Admin"),
            Role::User=> write!(f,"User")
        }
    }
}

#[derive(Serialize,Deserialize,Clone,FromRequest)]
pub struct UserClaims  {
    pub id: String,
    pub role: Role
}







