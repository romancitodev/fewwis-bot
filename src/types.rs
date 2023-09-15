pub struct Data;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type FnCommands = Vec<fn() -> poise::Command<Data, Error>>;
pub type Commands = Vec<poise::Command<Data, Error>>;
