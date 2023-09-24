pub use sea_orm_migration::prelude::*;

mod m20230920_015120_create_users;
mod m20230920_015138_create_stats;
mod m20230920_015144_create_rel_users_stats;
mod m20230920_015156_create_flags_stats;
mod m20230920_015202_create_rel_flags_stats;
mod m20230924_061320_buttons;
mod m20230924_061332_create_rel_buttons;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230920_015120_create_users::Migration),
            Box::new(m20230920_015138_create_stats::Migration),
            Box::new(m20230920_015144_create_rel_users_stats::Migration),
            Box::new(m20230920_015156_create_flags_stats::Migration),
            Box::new(m20230920_015202_create_rel_flags_stats::Migration),
            Box::new(m20230924_061320_buttons::Migration),
            Box::new(m20230924_061332_create_rel_buttons::Migration),
        ]
    }
}
