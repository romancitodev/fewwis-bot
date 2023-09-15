use crate::types::Commands;

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
    export_commands![utilities].iter().map(|e| e()).collect()
}
