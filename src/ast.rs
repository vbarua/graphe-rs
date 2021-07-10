#![allow(dead_code)]

use std::fmt::Display;

use crate::attributes::{Color, RankDir, Shape, Size};

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
    Length(f64),
    RankDir(RankDir),
    StyleNode(NodeStyle),
    Shape(Shape),
    Size(Size),
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
