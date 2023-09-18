use crate::types::Commands;

// Categorias
mod fun;
mod todos;
mod utilities;

macro_rules! export_commands {
    ($($cmd:ident),*) => { {
        let mut commands = vec![];
        $(
            commands.append(&mut $cmd::commands());
        )*
        commands
    }}
}

pub fn all() -> Commands {
    export_commands![utilities, todos, fun]
        .iter()
        .map(|e| e())
        .collect()
}
