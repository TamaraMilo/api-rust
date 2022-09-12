use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use crate::response_dto::ErrorDto;

#[derive(Debug, Error, Display)]
pub enum BucketError {
    #[display(fmt = "Error accrued while creating a bucket.")]
    BucketCreateError,
    #[display(fmt = "Error accrues while deleting bucket.")]
    BucketDeleteError,
    #[display(fmt="Bucket with that name already exists.")]
    BucketNameError,
    Unauthorized,
}

impl error::ResponseError for BucketError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorDto {
            message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            BucketError::BucketCreateError => StatusCode::BAD_REQUEST,
            BucketError::BucketDeleteError => StatusCode::BAD_REQUEST,
            BucketError::BucketNameError=> StatusCode::BAD_REQUEST,
            BucketError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
