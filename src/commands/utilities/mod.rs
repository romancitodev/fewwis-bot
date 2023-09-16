use crate::types::FnCommands;

mod age;
mod avatar;
mod ping;
mod sum;

pub fn commands() -> FnCommands {
    vec![
        age::get_age,
        ping::ping,
        sum::sum,
        avatar::avatar_ctx_menu,
        avatar::avatar,
    ]
}
