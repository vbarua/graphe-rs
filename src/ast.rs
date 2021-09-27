use crate::attributes::{Color, Label, RankDir, Shape, Size, Style};

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
        id: Id,
        attributes: Vec<Attribute>,
    },
    Edge {
        from: Id,
        to: Id,
        attributes: Vec<Attribute>,
    },
    Subgraph {
        id: Option<Id>,
        statements: Vec<Statement>,
    },
}

pub struct Id(pub(crate) String);

impl Id {
    pub fn new<T: ToString>(s: T) -> Self {
        Id(s.to_string())
    }

    pub(crate) fn get(&self) -> &str {
        self.0.as_str()
    }
}

pub trait ToId {
    fn to_id(&self) -> Id;
}

impl ToId for Id {
    fn to_id(&self) -> Id {
        Id(self.get().to_owned())
    }
}

impl ToId for u8 {
    fn to_id(&self) -> Id {
        Id(self.to_string())
    }
}

impl ToId for u16 {
    fn to_id(&self) -> Id {
        Id(self.to_string())
    }
}

impl ToId for u32 {
    fn to_id(&self) -> Id {
        Id(self.to_string())
    }
}

impl ToId for u64 {
    fn to_id(&self) -> Id {
        Id(self.to_string())
    }
}

impl ToId for &str {
    fn to_id(&self) -> Id {
        Id(self.to_owned().to_string())
    }
}

pub(crate) enum Attribute {
    Color(Color),
    Label(Label),
    Length(f64),
    RankDir(RankDir),
    Style(Style),
    Shape(Shape),
    Size(Size),
}
