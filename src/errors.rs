
use actix_web::{error, HttpResponse, http::StatusCode};
use derive_more::{Display,Error};

#[derive(Debug, Error, Display)]
pub enum Errors {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    DatabaseError,
    #[display(fmt = "An error occurred while writing a file.")]
    WritingFileError,
    #[display(fmt = "An error occurred while deleting a file.")]
    DeletingFileError,
    #[display(fmt = "An error occurred while changing a file")]
    ChangeFileError,
    #[display(fmt = "File not found")]
    NoFileError,
    #[display(fmt= "Error accrued while creating a bucket.")]
    BucketCreateError,
    #[display(fmt="Error creating file. Specified bucket does not exist")]
    BucketNotExisting,
    #[display(fmt="Error accrues while deleting bucket.")]
    BucketDeleteError,
    #[display(fmt= "Bucket do not exist.")]
    NoBucketError,
    #[display(fmt="Error creating cookie")]
    CreatingCookieError,
    #[display(fmt="Error logging in")]
    NoUser,
    #[display(fmt="Incorrect password or username or email.")]
    IncorrectUserPassword,
    #[display(fmt="You not logged in.")]
    LoggedOut,
    #[display(fmt="Password Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces. Password mut be at least 8 characters long. Username must contain number and alphabets only and must be at least 6 characters long ")]
    PassAndUsernameError,
    #[display(fmt="User with that username or email already exist.")]
    UserExistError,
    Unauthorized,
   
}   
impl error::ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Errors::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::WritingFileError => StatusCode::BAD_REQUEST,
            Errors::DeletingFileError => StatusCode::BAD_REQUEST,
            Errors::ChangeFileError => StatusCode::BAD_REQUEST,
            Errors::NoFileError => StatusCode::NOT_FOUND,
            Errors::BucketCreateError => StatusCode::BAD_REQUEST,
            Errors::BucketNotExisting => StatusCode::NOT_FOUND,
            Errors::BucketDeleteError => StatusCode::BAD_REQUEST,
            Errors::NoBucketError => StatusCode::NOT_FOUND,
            Errors::CreatingCookieError =>StatusCode::BAD_REQUEST,
            Errors::IncorrectUserPassword => StatusCode::BAD_REQUEST,
            Errors::NoUser => StatusCode::BAD_REQUEST,
            Errors::LoggedOut => StatusCode::BAD_REQUEST,
            Errors::PassAndUsernameError => StatusCode::BAD_REQUEST,
            Errors::Unauthorized => StatusCode::UNAUTHORIZED,
            Errors::UserExistError => StatusCode::BAD_REQUEST,
        }
    }
}
