use crate::types::FnCommands;

mod age;
mod ping;

pub fn commands() -> FnCommands {
    vec![age::get_age, ping::ping]
}
