use poise::serenity_prelude::Color;

pub enum Colors {
    White,
    Gray,
    Green,
    Orange,
    Red,
    Custom(u8, u8, u8),
}

impl From<Colors> for Color {
    fn from(value: Colors) -> Self {
        match value {
            Colors::White => Color::from_rgb(255, 255, 255),
            Colors::Gray => Color::from_rgb(175, 175, 175),
            Colors::Green => Color::from_rgb(178, 247, 117),
            Colors::Orange => Color::from_rgb(247, 194, 131),
            Colors::Red => Color::from_rgb(247, 131, 131),
            Colors::Custom(r, g, b) => Color::from_rgb(r, g, b),
        }
    }
}
