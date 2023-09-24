use crate::types::FnCommands;

mod memes;
mod quiz_buttons;
mod quiz_flags;

pub fn commands() -> FnCommands {
    vec![memes::meme, quiz_flags::quiz]
}
