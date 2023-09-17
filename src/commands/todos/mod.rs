use crate::types::FnCommands;

mod create;
mod status;

pub fn commands() -> FnCommands {
    vec![create::create_ctx_menu, create::todo, status::status]
}
