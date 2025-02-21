use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailSubscriber::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailSubscriber::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(EmailSubscriber::Name))
                    .col(ColumnDef::new(EmailSubscriber::Email).string().unique_key().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailSubscriber::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EmailSubscriber {
    Table,
    Id,
    Name,
    Email,
}
