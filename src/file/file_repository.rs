use crate::repository::Reposiory;
use async_trait::async_trait;
use entity::info::Entity as FileEntity;
use entity::info::Model as FileModel;
use migration::Condition;
use migration::DbErr;
use sea_orm::ColumnTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel, Set,
};

use super::dto::FileInfoDTO;

pub struct FileInfo {
    conn: DatabaseConnection,
}

#[async_trait]
impl Reposiory<FileModel, FileInfoDTO> for FileInfo {
    async fn create(&self, dto: FileInfoDTO) -> Result<FileModel, DbErr> {
        let file = FileModel {
            id: dto.id.to_string(),
            extension: dto.extension.to_string(),
            path: dto.path.to_string(),
            user_id: dto.user_id.to_string(),
            bucket_id: dto.bucket_id.to_string(),
        }
        .into_active_model();
        file.insert(&self.conn).await
    }

    async fn read(&self, id: String) -> Result<FileModel, DbErr> {
        let file = FileEntity::find_by_id(id).one(&self.conn).await?;
        match file {
            Some(r) => Ok(r),
            None => Err(DbErr::Custom("No file".to_string())),
        }
    }

    async fn update(&self, dto: FileInfoDTO) -> Result<FileModel, DbErr> {
        let file = Self::read(self, dto.id.to_string()).await?;
        let mut file = file.into_active_model();
        file.extension = Set(dto.extension.to_string());
        file.path = Set(dto.path.to_string());
        file.update(&self.conn).await
    }

    async fn delete(&self, id: String) -> Result<DeleteResult, DbErr> {
        FileEntity::delete_by_id(id).exec(&self.conn).await
    }
    fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
 
}

impl FileInfo {
    pub async fn read_files_from_bucket(&self, bucket_id: String) -> Result<Vec<FileModel>, DbErr> {
        let condition = Condition::all().add(entity::info::Column::BucketId.eq(bucket_id));
        FileEntity::find().filter(condition).all(&self.conn).await
    }
 
    pub async fn read_page(&self, bucket_id: String, page_number: usize) -> Result<Vec<FileModel>, DbErr> {
        let condition = Condition::all().add(entity::info::Column::BucketId.eq(bucket_id));
        FileEntity::find().filter(condition).paginate(&self.conn, 10).fetch_page(page_number).await

    } 
}
