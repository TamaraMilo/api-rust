use std::path::StripPrefixError;

use actix_easy_multipart::{File as OtherFile, FromMultipart};

pub struct FileInfoDTO
{
    pub id: String,
    pub extension: String,
    pub path: String,
    pub user_id: String,
    pub bucket_id: String,

}

#[derive(FromMultipart,Debug)]
pub struct ChangeFile {
   pub image: OtherFile,
}