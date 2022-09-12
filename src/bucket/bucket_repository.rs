use entity::bucket::Model;
use migration::{DbErr, Condition};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, ActiveModelTrait, DeleteResult, ColumnTrait, QueryFilter};
use entity::bucket::Entity as BucketEntity;
use crate::repository::Reposiory;
use async_trait::async_trait;

use super::dto::BucketDTO;

pub struct Bucket {
    pub conn: DatabaseConnection
}

#[async_trait]
impl Reposiory<Model,BucketDTO> for Bucket
{
  
    async fn create(&self,dto: BucketDTO) -> Result<Model,DbErr> {
     
        let bucket = Model{
            bucket_id: dto.bucket_id.to_string(),
            user_id: dto.user_id.to_string(),
            name: dto.name.to_string()
        }.into_active_model();

        bucket.insert(&self.conn).await 
    }


    async fn read(&self,name:String) -> Result<Model,DbErr>{
        let condition = Condition::any().add(entity::bucket::Column::Name.eq(name));

        let bucket = BucketEntity::find().filter(condition).one(&self.conn).await?;
        match bucket {
            Some(r)=> Ok(r),
            None => Err(DbErr::Custom("Nobucket".to_string()))
        }
    }

    async fn update(&self, _dto: BucketDTO) -> Result<Model,DbErr> {
        todo!()
    }

    async fn delete(&self,id: String) ->Result<DeleteResult,DbErr> {
        BucketEntity::delete_by_id(id).exec(&self.conn).await
    }

    
    fn new(conn: DatabaseConnection) -> Self{
        Self { conn: conn }
    }
   
}
impl Bucket {
    pub async fn bucket_exist(&self, naziv: String) -> Result<Option<Model>, DbErr>
    {
        let condition = Condition::any()
            .add(entity::bucket::Column::Name.eq(naziv));
        
        BucketEntity::find().filter(condition).one(&self.conn).await.map_err(|_| return DbErr::RecordNotFound("No bucket".to_string()))
    }

}