use std::fmt::Display;

pub enum Color {
    Blue,
    LightGrey,
    Red,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::Blue => "blue",
            Color::LightGrey => "lightgrey",
            Color::Red => "red",
            Color::White => "white",
        };
        f.write_str(s)
    }
}
