

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table( Bucket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bucket::BucketId)
                            .not_null()
                            .primary_key()
                            .string(),
                    )

                        .col(ColumnDef::new(Bucket::UserId)
                            .string()
                            .not_null())
                        .col(ColumnDef::new(Bucket::Name)
                            .string()
                            .not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bucket::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Bucket {
    Table,
    BucketId,
    UserId,
    Name
}
