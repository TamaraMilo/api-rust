pub use sea_orm_migration::prelude::*;


mod m20220810_000001_create_info_table;
mod m20220822_000001_create_user_table;
mod m20220823_000001_create_user_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220810_000001_create_info_table::Migration),
            Box::new(m20220822_000001_create_user_table::Migration),
            Box::new(m20220823_000001_create_user_table::Migration)]
    }
}