


use actix_web::{error, HttpResponse, http::StatusCode};
use derive_more::{Display,Error};
use crate::responses::ErrorDto;

#[derive(Debug, Error, Display)]
pub enum UserError {
    #[display(fmt="User do not exists.")]
    NoUser,
}

impl error::ResponseError for UserError
{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorDto{
            message: self.to_string()
        })
    }

    fn status_code(&self) -> StatusCode {

        match *self
        {
            UserError::NoUser => StatusCode::NOT_FOUND
        }
    }

}