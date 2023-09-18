use crate::types::FnCommands;

mod age;
mod avatar;
mod ping;
mod sum;
mod translate;

pub fn commands() -> FnCommands {
    vec![
        age::get_age,
        ping::ping,
        sum::sum,
        avatar::avatar_ctx_menu,
        avatar::avatar,
        translate::translate_ctx_menu,
        translate::translate,
    ]
}
