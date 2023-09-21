use crate::types::FnCommands;

mod memes;
mod quiz;

pub fn commands() -> FnCommands {
    vec![memes::meme, quiz::quiz]
}
