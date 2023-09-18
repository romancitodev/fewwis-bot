use crate::types::FnCommands;

mod memes;

pub fn commands() -> FnCommands {
    vec![memes::meme]
}
