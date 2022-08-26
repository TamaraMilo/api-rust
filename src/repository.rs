use sea_orm::DatabaseConnection;

pub trait Reposiory {
    fn create(&self, conn: &DatabaseConnection) -> Self;
    fn read(&self, conn: &DatabaseConnection) -> Self;
    fn update(&self, conn: &DatabaseConnection) -> Self;
    fn delete(&self, conn: &DatabaseConnection) -> Self;
}