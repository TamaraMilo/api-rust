

use actix_web::{error, HttpResponse, http::StatusCode};
use derive_more::{Display,Error};

use crate::responses::ErrorDto;

#[derive(Debug, Error, Display)]
pub enum FileError {
    #[display(fmt = "An error occurred while deleting a file.")]
    DeletingFileError,
    #[display(fmt = "An error occurred while changing a file")]
    ChangeFileError,
    #[display(fmt = "File not found")]
    NoFileError,
    Unauthorized,
    #[display(fmt = "Creating file error")]
    CreateFileError,
    #[display(fmt = "Bucket doesn't exist.")]
    NoBucketError,
    #[display(fmt = "Reading bucket error.")]
    ReadingFileError,
}
impl error::ResponseError for FileError
{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorDto{message: self.to_string()})
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self{
            FileError::DeletingFileError => StatusCode::BAD_REQUEST,
            FileError::ChangeFileError => StatusCode::BAD_REQUEST,
            FileError::NoFileError => StatusCode::NOT_FOUND,
            FileError::Unauthorized => StatusCode::UNAUTHORIZED,
            FileError::CreateFileError => StatusCode::BAD_REQUEST,
            FileError::NoBucketError => StatusCode::BAD_REQUEST,
            FileError::ReadingFileError => StatusCode::BAD_REQUEST
        }
    }
}