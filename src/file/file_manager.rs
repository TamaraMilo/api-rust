use actix_easy_multipart::{File as OtherFile, FromMultipart};
use entity::info::Model;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

#[derive(FromMultipart, Debug)]
pub struct UploadData {
    pub image: OtherFile,
    pub bucket: String,
}

pub struct FileManager {
    pub id: Uuid,
    pub buf: Vec<u8>,
    pub extension: String,
    pub path: String,
}

pub struct ChangeArgs<'a> {
    pub data: &'a mut OtherFile,
    pub file_info: &'a Model,
}

impl FileManager {
    pub fn new(file: &mut OtherFile, bucket_id: String, storage: &str) -> std::io::Result<Self> {
        let id = Uuid::new_v4();
        let extension = match file.get_extension() {
            Some(r) => r.to_string(),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Error getting file name",
                ))
            }
        };
        let path = format!("{}{}/{}.{}", storage, bucket_id, id, extension);

        let mut buf = Vec::new();
        file.file.read_to_end(&mut buf)?;
        let mut new_file = File::create(&path)?;
        new_file.write_all(&buf)?;

        Ok(Self {
            id,
            buf,
            path,
            extension,
        })
    }

    pub fn delete(path: String) -> Result<(), std::io::Error> {
        fs::remove_file(path)
    }

    pub fn change_data(arguments: ChangeArgs) -> Result<FileManager, std::io::Error> {
        fs::remove_file(arguments.file_info.path.clone())?;
        let extension = match arguments.data.get_extension() {
            Some(r) => r.to_string(),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Error getting file name",
                ))
            }
        };
        let tmp_path: Vec<&str> = arguments.file_info.path.split(".").collect();
        let new_path = format!("{}.{}", tmp_path[0], extension);
        let mut buf: Vec<u8> = Vec::new();
        arguments.data.file.read_to_end(&mut buf)?;
        let mut file = File::create(new_path.clone())?;
        file.write_all(&buf)?;

        let id = Uuid::parse_str(&arguments.file_info.id).map_err(|_| {
            return std::io::Error::new(std::io::ErrorKind::InvalidData, "Error getting file name");
        })?;

        Ok(FileManager {
            id: id,
            path: new_path,
            extension: extension,
            buf,
        })
    }

    pub fn show_file(storage: String, bucket: String, file:String) -> Result<Vec<u8>, std::io::Error> 
    {

        let path = format!("{}{}/{}", storage, bucket, file);
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
