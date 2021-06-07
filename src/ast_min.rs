pub(crate) struct Graph {
    strict: bool,
    id: Option<String>,
    statements: Vec<Statement>,
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

pub(crate) enum RankDir {
    TopBottom,
    LeftRight,
    BottomTop,
    RightLeft,
}
pub(crate) enum Shape {
    Box,
    Circle,
    DoubleCircle,
    MDiamond,
    MSquare,
}

pub(crate) struct Size {
    width: f64,
    height: f64,
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
