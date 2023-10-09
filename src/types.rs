use sea_orm::DatabaseConnection;
#[allow(dead_code)]
pub struct Data {
    pub db: DatabaseConnection,
}

impl Data {
    pub fn new(db: DatabaseConnection) -> Data {
        Data { db }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
pub type FnCommands = Vec<fn() -> poise::Command<Data, Error>>;
pub type Commands = Vec<poise::Command<Data, Error>>;
