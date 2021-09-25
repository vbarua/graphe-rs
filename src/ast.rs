use crate::attributes::{Color, RankDir, Shape, Size, Style};

pub(crate) enum GraphType {
    Directed,
    Undirected,
}

pub struct Graph {
    pub(crate) strict: bool,
    pub(crate) gtype: GraphType,
    pub(crate) id: Option<String>,
    pub(crate) statements: Vec<Statement>,
}

pub(crate) enum AttributeScope {
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
    Length(f64),
    RankDir(RankDir),
    Style(Style),
    Shape(Shape),
    Size(Size),
}
