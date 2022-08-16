static DEFAULT_PATH: &str = "./storage/";

use crate::errors::CustomError;

use std::io::prelude::*;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::fs::File;
use std::fs;
use uuid::Uuid;
use actix_web::web::Bytes;
pub struct FileInfo {
    pub id: Uuid,
    pub data: Vec<Bytes>,
    pub extension: String,
    pub path: String,
}
pub struct IdAndPath {
    pub id:Uuid,
    pub path: String,
}

pub struct ExtentionAndBytes {
    pub extension:String,
    pub bytes: Vec<Bytes>,
}
pub struct ChangeArgs {
    pub data: Vec<Bytes>,
    pub extension: String,
    pub path: String,
}


impl FileInfo {

    fn new() -> FileInfo {
        FileInfo{
            id: Uuid::new_v4(),
            data: Vec::new(),
            extension: String::new(),
            path: String::new(),
        }
    }

    pub fn new_file( data: Vec<Bytes>, extension: String) -> Result<IdAndPath, CustomError> {
        let mut new_file = FileInfo::new();
        new_file.data = data;
        new_file.extension = extension;
        new_file.path = format!("{}{}.{}",DEFAULT_PATH, new_file.id, new_file.extension);
        
        let mut file = File::create(&new_file.path).unwrap();

        for chunk in new_file.data {
            match file.write_all(chunk.as_ref())  {
                Ok(_file) => (),
                Err(_) => return Err(CustomError::WritingFileError),
            };
        }   
        return Ok(
            IdAndPath { 
                id: new_file.id, 
                path: new_file.path
            });
        
    }

    pub async fn parse_file_data(mut payload: Multipart) -> ExtentionAndBytes {
        let mut all_data:Vec<Bytes> = Vec::new();

        let mut field= payload.try_next().await.unwrap().unwrap();
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap().to_string();
        let extension: Vec<&str> = filename.split(".").collect();
        while let Some(chunk) = field.try_next().await.unwrap() {
            all_data.push(chunk);
        };

        ExtentionAndBytes{
            extension: extension[1].to_string(),
            bytes: all_data,
        }
    }

    pub fn delete(path: String) -> Result<String, CustomError> {
        match fs::remove_file(path) {
            Ok(_) => return Ok("File successfully deleted".to_string()),
            Err(_) => return Err(CustomError::DeletingFileError),
        };
    }

    pub fn change_data(arguments:ChangeArgs ) -> Result<String,CustomError> {
        
        match fs::remove_file(arguments.path.clone()) {
            Ok(_r) => (),
            Err(_) => return Err(CustomError::ChangeFileError),
        };

        let tmp_path:Vec<&str> = arguments.path.split(".").collect();
        let new_path = format!(".{}.{}",tmp_path[1],arguments.extension);
        let mut file = File::create(new_path.clone()).unwrap();

        for chunk in arguments.data 
        {
            match file.write_all(chunk.as_ref()) {
                Ok(_r) => (),
                Err(_) => return Err(CustomError::ChangeFileError),
            }
        };

        return Ok(new_path);
    }

  

}