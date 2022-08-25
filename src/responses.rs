use std::fmt;

use entity::user::Role;
use serde::{Serialize, Deserialize};

pub enum ResponseText {
    LoggedIn,
    SingIn,
    LoggedOut
}

impl fmt::Display for ResponseText {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            ResponseText::LoggedIn => write!(f,"You are logged in."),
            ResponseText::SingIn=> write!(f,"You are singed in."),
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

#[derive(Serialize)]
pub struct Response<T>
{
    pub data: Option<T>,
    pub errors: Option<T>
}
//response
#[derive(Serialize)]
pub struct FileDetailsResponse {
    pub id: String,
    pub extension:String,
    pub path:String,
}