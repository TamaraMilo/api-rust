

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug,Error,Serialize,Deserialize)]
pub enum DatabaseError {
    #[error("Database error.")]
    DatabaseError,
}   
impl DatabaseError {
    pub fn error_response(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug,Error,Serialize,Deserialize)]
pub enum FileError {
    #[error("Error occured while writing data in file.")]
    WritingFileError,
    #[error("Error occcrued while deleting a file.")]
    DeletingFileError,
    #[error("Error occrued while changing data in file.")]
    ChangeFileError,
    #[error("File do not exist.")]
    NoFileError,
   
}
impl FileError {
    pub fn error_response(&self) -> String {
        self.to_string()
    }
}


#[derive(Debug,Error,Serialize,Deserialize)]
pub enum BucketError {
    #[error("Error accrued while creating a bucket.")]
    BucketCreateError,
    #[error("Error creating file. Specified bucket does not exist")]
    BucketNotExisting,
    #[error("Error accrues while deleting bucket.")]
    BucketDeleteError,
    #[error("Bucket do not exist.")]
    NoBucketError,
   
}
impl BucketError {
    pub fn error_response(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug,Error,Serialize,Deserialize)]
pub enum LoggingError {
    #[error("Error creating cookie")]
    CreatingCookieError,
    #[error("Error logging in")]
    NoUser,
    #[error("Incorrect password or username or email.")]
    IncorrectUserPassword,
    #[error("You not logged in.")]
    LoggedOut,
    #[error("Password Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces. Password mut be at least 8 characters long. Username must contain number and alphabets only and must be at least 6 characters long ")]
    PassAndUsernameError,
    #[error("User with that username already exist.")]
    UsernameError,
    #[error("User with that email already exist")]
    EmailError
}
impl LoggingError {
    pub fn error_response(&self) -> String {
        self.to_string()
    }
}

