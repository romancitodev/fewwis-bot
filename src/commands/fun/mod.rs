use crate::types::FnCommands;

mod memes;
mod quiz;
mod quiz_buttons;

pub fn commands() -> FnCommands {
    vec![memes::meme, quiz::quiz]
}
