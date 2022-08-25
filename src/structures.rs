use actix_easy_multipart::{FromMultipart, File};
use actix_jwt_auth_middleware::Authority;
use entity::user::Role;
use regex::Regex;
use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::auth::UserClaims;


#[derive(Clone)]
pub struct AppState{
    pub conn: DatabaseConnection,
    pub env_data: EnvData,
    pub auth: Authority<UserClaims>,
}
#[derive(Serialize, Deserialize, Validate)]
pub struct UserData {
    #[validate(regex="USERNAME_REGEX")]
    pub username: String,
    #[validate(email)]
    pub email:String,
    #[validate(custom(function ="is_password_valid"))]
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct LogginData {
    pub identifier: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: Role,
}
lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"[a-zA-Z0-9][a-zA-Z0-9_]{7,}$").unwrap();
}


fn is_password_valid(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {

        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }

    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    }else {
        return Err(ValidationError::new("terrible_password"));
    }
}


pub fn validate(id: String, user_claims_id: String, user_role: Role) -> bool {
    if id != user_claims_id {
        if user_role != Role::Admin {
          return false;
        }
    }
    true
}

#[derive(Debug,Clone)]
pub struct EnvData{
    pub database_url:  String,
    pub basic_storage: String,
    pub max_transfer_size: usize,
  
}

impl EnvData {
    pub fn load()-> std::io::Result<Self> {   
        let database_url = dotenv::var("DATABASE_URL").unwrap();
        let basic_storage = dotenv::var("BASIC_STORAGE").unwrap();
        let max_transfer_size = dotenv::var("MAX_TRANSFER_SIZE").unwrap().parse::<i32>().unwrap().try_into().unwrap(); 
        Ok(Self{
            database_url,
            basic_storage,
            max_transfer_size,
        })
    }
}
#[derive(FromMultipart,Debug)]
pub struct Upload {
   pub image: File,
}
#[derive(Serialize)]
pub struct Response<T>
{
    pub data: Option<T>,
    pub errors: Option<T>
}
#[derive(Serialize)]
pub struct FileDetails {
    pub id: String,
    pub extension:String,
    pub path:String,
}