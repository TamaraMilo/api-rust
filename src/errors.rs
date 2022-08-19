
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug,Error,Serialize,Deserialize)]
pub enum CustomError {
    #[error("Error accrued while writing data in file.")]
    WritingFileError,
    #[error("Error acccrued while deleting a file.")]
    DeletingFileError,
    #[error("Error accrued while changing data in file.")]
    ChangeFileError,
    #[error("Error fetching data from database. File do not exist.")]
    NoFileError,
    #[error("Error accrued while creating a bucket.")]
    BucketCreateError,
    #[error("Error creating file. Specified bucket does not exist")]
    BucketNotExisting,
    #[error("Error accrues while deleting bucket.")]
    BucketDeleteError,
    #[error("Error loading data from database.")]
    DatabaseError,
    #[error("Error accrued while savinga data in database.")]
    SavigError,
}   
impl CustomError {
    pub fn error_response(&self) -> String {
        self.to_string()
    }
}