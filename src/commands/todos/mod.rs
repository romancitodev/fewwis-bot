use crate::types::FnCommands;

mod create;

pub fn commands() -> FnCommands {
    vec![create::create_ctx_menu, create::todo, create::create]
}
