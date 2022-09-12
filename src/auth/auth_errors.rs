use actix_web::{error, HttpResponse, http::StatusCode};
use derive_more::{Display,Error};
use crate::response_dto::ErrorDto;


#[derive(Debug, Error, Display)]
pub enum AuthError {
    #[display(fmt="Incorrect password or username or email.")]
    IncorrectUserPassword,
    #[display(fmt="Password Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces. Password mut be at least 8 characters long. Username must contain number and alphabets only and must be at least 6 characters long ")]
    PassAndUsernameError,
    #[display(fmt="Sign in error.")]
    SignInError,
    #[display(fmt=" Sign up error.")]
    SingUpError
    
}

impl error::ResponseError for AuthError {

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorDto{
            message: self.to_string()
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::IncorrectUserPassword => StatusCode::BAD_REQUEST,
            AuthError::PassAndUsernameError => StatusCode::BAD_REQUEST,
            AuthError::SignInError => StatusCode::BAD_REQUEST,
            AuthError::SingUpError => StatusCode::BAD_REQUEST

        }
    }

}