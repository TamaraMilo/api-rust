
use std::fs;
use std::fs::File;
use std::io::{prelude::*};
use entity::info::Model;
use uuid::Uuid;
use actix_easy_multipart::{File as OtherFile, FromMultipart};


#[derive(FromMultipart,Debug)]
pub struct UploadData {
   pub image: OtherFile,
}



pub struct FileInfo {
    pub id: Uuid,
    pub buf: Vec<u8>,
    pub extension: String,
    pub path: String,
}

pub struct ChangeArgs<'a> {
    pub data: &'a mut OtherFile,
    pub file_info: &'a Model 
}

impl FileInfo {
    pub fn new(file: &mut OtherFile,bucket_id: String,storage: &str ) -> std::io::Result<Self> {
        
        let id = Uuid::new_v4();
        let extension = match file.get_extension() {
            Some(r) => r.to_string(),
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error getting file name"))
        };
        let path = format!("{}{}/{}.{}",storage,bucket_id, id, extension);   
        
        let mut buf = Vec::new();
        file.file.read_to_end(&mut buf)?;
        let mut new_file = File::create(&path)?;
        new_file.write_all(&buf)?;
        

        Ok(Self{
            id,
            buf,
            path,
            extension
        })
    }

    pub fn delete(path: String) -> Result<(), std::io::Error> {
        fs::remove_file(path)
    }

    pub fn change_data(arguments: ChangeArgs) -> Result<FileInfo, std::io::Error> {
        fs::remove_file(arguments.file_info.path.clone())?;
        let extension = match arguments.data.get_extension(){
            Some(r) => r.to_string(),
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error getting file name"))
        };
        let tmp_path: Vec<&str> = arguments.file_info.path.split(".").collect();
        let new_path = format!("{}.{}", tmp_path[0], extension);
        let mut buf:Vec<u8> = Vec::new();
        arguments.data.file.read_to_end(&mut buf)?;
        let mut file = File::create(new_path.clone())?;
        file.write_all(&buf)?;

        let  id =match  Uuid::parse_str(&arguments.file_info.id) {
            Ok(r)=>r,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error getting file name"))
        };

        Ok(FileInfo{
            id: id,
            path: new_path,
            extension: extension,
            buf,
        })
    }
   
}
