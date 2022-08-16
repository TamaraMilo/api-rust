pub use sea_orm_migration::prelude::*;

mod m20220810_000001_create_info_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220810_000001_create_info_table::Migration)]
    }
}