use sea_orm_migration::prelude::*;

use crate::{m20230920_015120_create_users::Users, m20230920_015138_create_stats::Stats};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Relation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Relation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Relation::StatsId).integer().not_null())
                    .col(ColumnDef::new(Relation::UsersId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rel-users")
                            .from(Relation::Table, Relation::Id)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rel-stats")
                            .from(Relation::Table, Relation::Id)
                            .to(Stats::Table, Stats::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Relation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Relation {
    #[sea_orm(iden = "rel-users-stats")]
    Table,
    Id,
    StatsId,
    UsersId,
}
