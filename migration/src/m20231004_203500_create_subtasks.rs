use sea_orm_migration::prelude::*;

use crate::m20231004_203431_create_tasks::Task;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Step::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Step::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Step::TaskId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel-post-subtask")
                            .from(Step::Table, Step::Id)
                            .to(Task::Table, Task::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Step::Index).big_integer().not_null())
                    .col(ColumnDef::new(Step::Description).string().not_null())
                    .col(ColumnDef::new(Step::Completed).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Step::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Step {
    Table,
    Id,
    TaskId,
    Index,
    Description,
    Completed,
}
