#![allow(dead_code)]

use std::io;

enum Graph {
    Undirected {
        strict: bool,
        id: Option<Identifier>,
        statements: Vec<Statement>,
    },
    Directed {
        strict: bool,
        id: Option<Identifier>,
        statements: Vec<Statement>,
    },
}

// stmt : node_stmt
//      | edge_stmt
//      | attr_stmt
//      | ID '=' ID
//      | subgraph
enum Statement {
    Node {
        id: Identifier,
        attributes: Vec<NodeAttribute>,
    },
    Edge {
        from: Identifier, // TODO: Model NodeID | Subgraph
        to: Identifier,
        attributes: Vec<EdgeAttribute>,
    },
    Attr(Vec<Attribute>),
    Equality(Identifier, Identifier),
    Subgraph {
        id: Identifier,
        statements: Vec<Statement>,
    },
}

enum Attribute {
    Graph(GraphAttribute),
    Node(NodeAttribute),
    Edge(EdgeAttribute),
}

enum GraphAttribute {
    RankDir(String),
    Size(f64),
}

enum NodeAttribute {
    Shape(String),
    Label(String),
}

enum EdgeAttribute {
    Label(String),
}

// TODO: All of them
type Identifier = String;

trait DotVisitor<T> {
    fn visit_graph(&mut self, graph: &Graph) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_graph_attribute(&mut self, graph_attribute: &GraphAttribute) -> T;
    fn visit_node_attribute(&mut self, node_attribute: &NodeAttribute) -> T;
    fn visit_edge_attribute(&mut self, edge_attribute: &EdgeAttribute) -> T;
}

struct PrettyPrinter<W: io::Write> {
    depth: i32,
    writer: W,
}

impl<W: io::Write> PrettyPrinter<W> {
    #[inline]
    fn indent(&mut self) -> io::Result<()> {
        for _ in 0..self.depth {
            self.writer.write_all(b"  ")?;
        }
        Ok(())
    }

    #[inline]
    fn arrow(&mut self) -> io::Result<()> {
        self.writer.write_all(b" -> ")
    }

    #[inline]
    fn newline(&mut self) -> io::Result<()> {
        self.writer.write_all(b"\n")
    }

    #[inline]
    fn whitespace(&mut self) -> io::Result<()> {
        self.writer.write_all(b" ")
    }

    #[inline]
    fn semicolon(&mut self) -> io::Result<()> {
        self.writer.write_all(b";\n")
    }
}

impl<W: io::Write> DotVisitor<io::Result<()>> for PrettyPrinter<W> {
    fn visit_graph(&mut self, graph: &Graph) -> io::Result<()> {
        let statements = match graph {
            Graph::Undirected {
                strict,
                id,
                statements,
            } => {
                if *strict {
                    self.writer.write_all(b"strict ")?;
                }
                self.writer.write_all(b"graph ")?;
                if let Some(ident) = id {
                    self.writer.write_all(ident.as_bytes())?;
                    self.whitespace()?
                }
                statements
            }
            Graph::Directed {
                strict,
                id,
                statements,
            } => {
                if *strict {
                    self.writer.write_all(b"strict ")?;
                }
                self.writer.write_all(b"digraph ")?;
                if let Some(ident) = id {
                    self.writer.write_all(ident.as_bytes())?;
                    self.whitespace()?;
                }
                statements
            }
        };
        self.writer.write_all(b"{")?;
        self.newline()?;
        self.depth += 1;
        for s in statements {
            self.visit_statement(s);
        }
        self.writer.write_all(b"}")?;
        self.newline()?;
        Ok(())
    }

    fn visit_statement(&mut self, statement: &Statement) -> io::Result<()> {
        match statement {
            Statement::Node { id, attributes } => {
                self.indent()?;
                self.writer.write_all(id.as_bytes())?;
                self.whitespace()?;
                self.writer.write_all(b"[")?;
                for attr in attributes {
                    self.visit_node_attribute(attr)?;
                }
                self.writer.write_all(b"]")?;
                self.semicolon()?;
            }
            Statement::Edge {
                from,
                to,
                attributes,
            } => {
                self.indent()?;
                self.writer.write_all(from.as_bytes())?;
                self.arrow()?;
                self.writer.write_all(to.as_bytes())?;
                match attributes.as_slice() {
                    [] => (),
                    [attr] => {
                        self.writer.write_all(b" [")?;
                        self.visit_edge_attribute(attr)?;
                        self.writer.write_all(b"]")?;
                    }
                    _ => {
                        self.writer.write_all(b" [")?;
                        for attr in attributes {
                            self.visit_edge_attribute(attr)?;
                        }
                        self.writer.write_all(b"]")?;
                    }
                }
                self.semicolon()?;
            }
            Statement::Attr(attributes) => {
                for attr in attributes {
                    match attr {
                        Attribute::Graph(g) => {
                            self.visit_graph_attribute(g)?;
                            self.newline()?;
                        }
                        Attribute::Node(n) => self.visit_node_attribute(n)?,
                        Attribute::Edge(e) => self.visit_edge_attribute(e)?,
                    }
                }
            }
            Statement::Equality(_, _) => {
                todo!()
            }
            Statement::Subgraph { id, statements } => {
                todo!();
            }
        }
        Ok(())
    }

    fn visit_graph_attribute(&mut self, graph_attribute: &GraphAttribute) -> io::Result<()> {
        self.indent()?;
        match graph_attribute {
            GraphAttribute::RankDir(layout) => {
                write!(self.writer, "rankdir={};", layout)?;
            }
            GraphAttribute::Size(s) => {
                write!(self.writer, "size={};", s)?;
            }
        }
        Ok(())
    }

    fn visit_node_attribute(&mut self, node_attribute: &NodeAttribute) -> io::Result<()> {
        match node_attribute {
            NodeAttribute::Shape(s) => {
                write!(self.writer, "shape = \"{}\"", s)?;
            }
            NodeAttribute::Label(_) => {}
        }
        Ok(())
    }

    fn visit_edge_attribute(&mut self, edge_attribute: &EdgeAttribute) -> io::Result<()> {
        match edge_attribute {
            EdgeAttribute::Label(l) => {
                write!(self.writer, "label = \"{}\"", l)?;
            }
        }
        Ok(())
    }
}

fn pretty_print<W: io::Write>(graph: &Graph, writer: W) -> io::Result<()> {
    let mut pp = PrettyPrinter {
        depth: 0,
        writer: writer,
    };
    pp.visit_graph(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn hello_world() {
        // https://graphviz.org/Gallery/directed/hello.html
        let graph = Graph::Directed {
            strict: false,
            id: Some("G".to_string()),
            statements: vec![Statement::Edge {
                from: "Hello".to_string(),
                to: "World".to_string(),
                attributes: vec![],
            }],
        };

        let writer = io::stdout();
        pretty_print(&graph, writer);
    }

    #[test]
    fn finite_state_machine() {
        let graph = Graph::Directed {
            strict: false,
            id: Some("finite_state_machine".to_string()),
            statements: vec![
                Statement::Attr(vec![
                    Attribute::Graph(GraphAttribute::RankDir("LR".to_string())),
                    Attribute::Graph(GraphAttribute::Size(8.5)),
                ]),
                Statement::Node {
                    id: "0".to_string(),
                    attributes: vec![NodeAttribute::Shape("doublecircle".to_string())],
                },
                Statement::Node {
                    id: "3".to_string(),
                    attributes: vec![NodeAttribute::Shape("doublecircle".to_string())],
                },
                Statement::Edge {
                    from: "0".to_string(),
                    to: "2".to_string(),
                    attributes: vec![EdgeAttribute::Label("SS(B)".to_string())],
                },
                Statement::Edge {
                    from: "0".to_string(),
                    to: "1".to_string(),
                    attributes: vec![EdgeAttribute::Label("SS(S)".to_string())],
                },
                Statement::Edge {
                    from: "1".to_string(),
                    to: "3".to_string(),
                    attributes: vec![EdgeAttribute::Label("S($end)".to_string())],
                },
            ],
        };

        let writer = io::stdout();
        let mut s = String::new();
        pretty_print(&graph, writer);
    }
}
