use sea_orm_migration::prelude::*;

use crate::{m20230920_015138_create_stats::Stats, m20230924_061320_buttons::Buttons};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(Relation::ButtonsId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rel-stats-buttons")
                            .from(Relation::Table, Relation::StatsId)
                            .to(Stats::Table, Stats::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rel-buttons-stats")
                            .from(Relation::Table, Relation::ButtonsId)
                            .to(Buttons::Table, Buttons::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Relation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Relation {
    #[sea_orm(iden = "rel-buttons-stats")]
    Table,
    Id,
    StatsId,
    ButtonsId,
}
