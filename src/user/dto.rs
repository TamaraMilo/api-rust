
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"[a-zA-Z0-9][a-zA-Z0-9_]{7,}$").unwrap();
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$").unwrap();
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct UserCreateDTO {
    #[validate(custom = "is_username_valid")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "is_password_valid")]
    pub password: String,
}


fn is_password_valid(password: &str) -> Result<(), ValidationError> {
    let valid: bool = PASSWORD_REGEX.is_match(password).unwrap();
    if valid {
        Ok(())
    } else {
        return Err(ValidationError::new("terrible_password"));
    }
}
fn is_username_valid(username: &str) -> Result<(), ValidationError> {
    let valid: bool = USERNAME_REGEX.is_match(username).unwrap();
    if valid {
        Ok(())
    } else {
        return Err(ValidationError::new("terrible_username"));
    }
}
