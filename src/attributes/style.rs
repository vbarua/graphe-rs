use std::fmt::Display;

// docs
// * https://graphviz.org/docs/attrs/style/
// * https://graphviz.org/docs/attr-types/style/

pub(crate) enum EdgeStyle {
    // Common to Edges and Nodes
    Bold,
    Dashed,
    Dotted,
    Invisible,
    Solid,
}

// https://graphviz.org/doc/info/shapes.html#styles-for-nodes
pub(crate) enum NodeStyle {
    // Common to Edges and Nodes
    Bold,
    Dashed,
    Dotted,
    Invisible,
    Solid,
    // Node Only
    Diagonals,
    Filled,
    Rounded,
}

pub(crate) enum ClusterStyle {
    Bold,
    Dashed,
    Dotted,
    Filled,
    Rounded,
    Solid,
}

pub(crate) enum Style {
    Bold,
    Dashed,
    Diagonals,
    Dotted,
    Filled,
    Invisible,
    Rounded,
    Solid,
}

impl From<EdgeStyle> for Style {
    fn from(es: EdgeStyle) -> Self {
        match es {
            EdgeStyle::Bold => Style::Bold,
            EdgeStyle::Dashed => Style::Dashed,
            EdgeStyle::Dotted => Style::Dotted,
            EdgeStyle::Invisible => Style::Invisible,
            EdgeStyle::Solid => Style::Solid,
        }
    }
}

impl From<NodeStyle> for Style {
    fn from(ns: NodeStyle) -> Self {
        match ns {
            NodeStyle::Bold => Style::Bold,
            NodeStyle::Dashed => Style::Dashed,
            NodeStyle::Dotted => Style::Dotted,
            NodeStyle::Invisible => Style::Invisible,
            NodeStyle::Solid => Style::Solid,
            NodeStyle::Diagonals => Style::Diagonals,
            NodeStyle::Filled => Style::Filled,
            NodeStyle::Rounded => Style::Rounded,
        }
    }
}

impl From<ClusterStyle> for Style {
    fn from(cs: ClusterStyle) -> Self {
        match cs {
            ClusterStyle::Bold => Style::Bold,
            ClusterStyle::Dashed => Style::Dashed,
            ClusterStyle::Dotted => Style::Dotted,
            ClusterStyle::Filled => Style::Filled,
            ClusterStyle::Rounded => Style::Rounded,
            ClusterStyle::Solid => Style::Solid,
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Style::Bold => "bold",
            Style::Dashed => "dashed",
            Style::Diagonals => "diagonals",
            Style::Dotted => "dotted",
            Style::Filled => "filled",
            Style::Invisible => "invis",
            Style::Rounded => "rounded",
            Style::Solid => "solid",
        };
        f.write_str(s)
    }
}
