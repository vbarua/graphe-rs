#![allow(dead_code)]

use std::fmt::Display;

pub struct Graph {
    pub(crate) strict: bool,
    pub(crate) id: Option<String>,
    pub(crate) statements: Vec<Statement>,
}

pub(crate) enum AttributeScope {
    Default,
    Graph,
    Node,
    Edge,
}

pub(crate) enum Statement {
    Attribute(AttributeScope, Vec<Attribute>),
    Node {
        id: String,
        attributes: Vec<Attribute>,
    },
    Edge {
        from: String,
        to: String,
        attributes: Vec<Attribute>,
    },
    Subgraph {
        id: Option<String>,
        statements: Vec<Statement>,
    },
}

pub(crate) enum Attribute {
    Color(Color),
    Label(String),
    RankDir(RankDir),
    StyleNode(NodeStyle),
    Shape(Shape),
    Size(Size),
}

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

pub(crate) enum RankDir {
    TopBottom,
    LeftRight,
    BottomTop,
    RightLeft,
}

impl Display for RankDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RankDir::TopBottom => "TB",
            RankDir::LeftRight => "LR",
            RankDir::BottomTop => "BT",
            RankDir::RightLeft => "RL",
        };
        f.write_str(s)
    }
}

pub(crate) enum Shape {
    Box,
    Circle,
    DoubleCircle,
    MDiamond,
    MSquare,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Shape::Box => "box",
            Shape::Circle => "circle",
            Shape::DoubleCircle => "doublecircle",
            Shape::MDiamond => "Mdiamond",
            Shape::MSquare => "Msquare",
        };
        f.write_str(s)
    }
}

pub(crate) struct Size {
    width: f64,
    height: f64,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{},{}\"", self.width, self.height)
    }
}

impl From<f64> for Size {
    fn from(f: f64) -> Self {
        Size {
            width: f,
            height: f,
        }
    }
}

impl From<(f64, f64)> for Size {
    fn from(fs: (f64, f64)) -> Self {
        Size {
            width: fs.0,
            height: fs.1,
        }
    }
}

pub(crate) enum NodeStyle {
    Filled,
}

impl Display for NodeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NodeStyle::Filled => "filled",
        };
        f.write_str(s)
    }
}
