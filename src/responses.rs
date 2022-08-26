use std::fmt;

use entity::user::{Role, Model};
use serde::{Serialize, Deserialize};

pub enum ResponseText {

    LoggedOut
}

impl fmt::Display for ResponseText {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            ResponseText::LoggedOut=> write!(f,"You are logged out")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: Role,
}
impl LoginResponse {
    pub fn new(user: Model) -> Self {
        Self{
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    } 
}


//response
#[derive(Serialize)]
pub struct FileDetailsResponse {
    pub id: String,
    pub extension:String,
    pub path:String,
}