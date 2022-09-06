use migration::DbErr;
use sea_orm::{DatabaseConnection, DeleteResult};
use async_trait::async_trait;

#[async_trait]
pub trait Reposiory<K, T>
{
    async fn create(&self, dto: T) -> Result<K,DbErr>;
    async fn read(&self, id: String) -> Result<K,DbErr>;
    async fn update(&self, dto: T) -> Result<K,DbErr>;
    async fn delete(&self, id: String) -> Result<DeleteResult,DbErr>;
    fn new(conn: DatabaseConnection) -> Self;
}