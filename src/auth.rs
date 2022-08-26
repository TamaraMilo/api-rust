use actix_jwt_auth_middleware::FromRequest;
use entity::user::Role;
use fancy_regex::Regex;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};


//auth
#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct SingInRequest {
    #[validate(custom="is_username_valid")]
    pub username: String,
    #[validate(email)]
    pub email:String,
    #[validate(custom="is_password_valid")]
    pub password: String,
}
//auth
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UserIdetifier{
    pub email: String,
    pub username: String,
}

#[derive(Serialize,Deserialize,Clone,FromRequest)]
pub struct UserClaims  {
    pub id: String,
    pub role: Role
}

pub fn validate(id: String, user_claims_id: String, user_role: Role) -> bool {
    if id != user_claims_id {
        if user_role != Role::Admin {
          return false;
        }
    }
    true
}

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"[a-zA-Z0-9][a-zA-Z0-9_]{7,}$").unwrap();
    static ref PASSWORD_REGEX:Regex = Regex::new(r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$").unwrap();
}

fn is_password_valid(password: &str) -> Result<(), ValidationError> {

    let valid: bool = PASSWORD_REGEX.is_match(password).unwrap();
    if valid {
        Ok(())
    }else {
        return Err(ValidationError::new("terrible_password"));
    }
}
fn is_username_valid(username: &str) -> Result<(), ValidationError> {

    let valid: bool = USERNAME_REGEX.is_match(username).unwrap();
    if valid {
        Ok(())
    }else {
        return Err(ValidationError::new("terrible_username"));
    }
}

