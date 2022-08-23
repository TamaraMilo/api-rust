use std::fs;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Bucket {
    pub id: Uuid,
}

impl Bucket {
    pub fn new(storage:&str) ->std::io::Result<Self>{
        let id = Uuid::new_v4();
        let path = format!("{}{}",storage,id);

        fs::create_dir(&path)?;
        Ok(Self{
            id,
            
        })
    }
    pub fn delete(bucket_id: String, storage:&str) -> Result<(),std::io::Error>
    {   let path = format!("{}{}",storage,bucket_id);
        fs::remove_dir(path)?;
        Ok(())
    }
}