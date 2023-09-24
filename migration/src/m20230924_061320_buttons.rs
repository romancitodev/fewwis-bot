use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Buttons::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Buttons::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Buttons::FirstAttempt).integer().not_null())
                    .col(ColumnDef::new(Buttons::SecondAttempt).integer().not_null())
                    .col(ColumnDef::new(Buttons::ThirdAttempt).integer().not_null())
                    .col(ColumnDef::new(Buttons::Wrong).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Buttons::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Buttons {
    Table,
    Id,
    FirstAttempt,
    SecondAttempt,
    ThirdAttempt,
    Wrong,
}