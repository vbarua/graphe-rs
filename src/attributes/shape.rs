use std::fmt::Display;

pub enum Shape {
    Box,
    Circle,
    Diamond,
    DoubleCircle,
    Ellipse,
    MDiamond,
    MSquare,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Shape::Box => "box",
            Shape::Circle => "circle",
            Shape::Diamond => "diamond",
            Shape::DoubleCircle => "doublecircle",
            Shape::Ellipse => "ellipse",
            Shape::MDiamond => "Mdiamond",
            Shape::MSquare => "Msquare",
        };
        f.write_str(s)
    }
}
