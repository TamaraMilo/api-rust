
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;
use actix_easy_multipart::File as OtherFile;



pub struct FileInfo {
    pub id: Uuid,
    pub buf: Vec<u8>,
    pub extension: String,
    pub path: String,
}

pub struct ChangeArgs<'a> {
    pub data: &'a mut OtherFile,
    pub path: String,
}
pub struct ChangedData{
    pub path: String,
    pub extension:String,
}


impl FileInfo {
    pub fn new(file: &mut OtherFile,bucket_id: String,storage: String ) -> std::io::Result<Self> {
        
        let id = Uuid::new_v4();
        let extension = file.get_extension().unwrap().to_string();
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
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn change_data(arguments: ChangeArgs) -> Result<ChangedData, std::io::Error> {
        fs::remove_file(arguments.path.clone())?;
        let extension = arguments.data.get_extension().unwrap().to_string();
        let tmp_path: Vec<&str> = arguments.path.split(".").collect();
        let new_path = format!("{}.{}", tmp_path[0], extension);
        let mut buf:Vec<u8> = Vec::new();
        arguments.data.file.read_to_end(&mut buf)?;
        let mut file = File::create(new_path.clone())?;
        file.write_all(&buf)?;
        

        Ok(ChangedData{
            path: new_path,
            extension: extension,
        })
    }
   
}
