use sea_orm::DatabaseConnection;



#[derive(Clone)]
pub struct AppState{
    pub conn: DatabaseConnection,
    pub env_data: EnvData,

}

#[derive(Debug,Clone)]
pub struct EnvData{
    pub database_url:  String,
    pub basic_storage: String,
    pub max_transfer_size: usize,
    pub secret_key: String,
    pub page_size: usize
  
}

impl EnvData {
    pub fn load()-> std::io::Result<Self> {   
        let database_url = dotenv::var("DATABASE_URL").unwrap();
        let basic_storage = dotenv::var("BASIC_STORAGE").unwrap();
        let max_transfer_size = dotenv::var("MAX_TRANSFER_SIZE").unwrap().parse::<i32>().unwrap().try_into().unwrap(); 
        let secret_key = dotenv::var("SECRET").unwrap();
        let page_size = dotenv::var("PAGE_SIZE").unwrap().parse::<i32>().unwrap().try_into().unwrap(); 
        Ok(Self{
            database_url,
            basic_storage,
            max_transfer_size,
            secret_key,
            page_size
        })
    }
}