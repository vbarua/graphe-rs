use std::fmt::Display;

pub(crate) enum Color {
    Blue,
    LightGrey,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::Blue => "blue",
            Color::LightGrey => "lightgrey",
            Color::White => "white",
        };
        f.write_str(s)
    }
}
