

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
    #[error("Error accrued while writing data in file.")]
    WritingFileError,
    #[error("Error acccrued while deleting a file.")]
    DeletingFileError,
    #[error("Error accrued while changing data in file.")]
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
    #[error("Incorrect password or username.")]
    IncorrectPassword,
    #[error("You not logged in.")]
    LoggedOut,
}
impl LoggingError {
    pub fn error_response(&self) -> String {
        self.to_string()
    }
}

