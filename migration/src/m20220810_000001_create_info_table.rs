use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Info::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Info::Id)
                            .not_null()
                            .primary_key()
                            .string(),
                    )
                    .col(ColumnDef::new(Info::Extension)
                            .string()
                            .not_null())
                    .col(ColumnDef::new(Info::Path)
                            .string()
                            .not_null())
                    .col(ColumnDef::new(Info::UserId)
                            .string()
                            .not_null())
                    .col(ColumnDef::new(Info::BucketID)
                            .string()
                            .not_null())
                    .to_owned(),

            )
            .await
      
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Info::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Info {
    Table,
    Id,
    Extension,
    Path,
    UserId,
    BucketID,
}
