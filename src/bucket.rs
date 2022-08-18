use std::fs;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Bucket {
    pub id: Uuid,
}

impl Bucket {
    pub fn new() ->std::io::Result<Self>{
        let id = Uuid::new_v4();
        let path = format!("{}{}",dotenv::var("BASIC_STORAGE").unwrap(),id);
        fs::create_dir(&path)?;
        Ok(Self{
            id,
          
        })
    }
    pub fn delete(bucket_id: String) -> Result<(),std::io::Error>
    {   let path = format!("{}{}",dotenv::var("BASIC_STORAGE").unwrap(),bucket_id);
        fs::remove_dir(path)?;
        Ok(())
    }
}