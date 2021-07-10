use std::io;

use crate::ast::{Attribute, AttributeScope, Graph, GraphType, Statement};

trait AstVisitor<T> {
    fn visit_graph(&mut self, graph: &Graph) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_attributes(&mut self, attributes: &[Attribute]) -> T;
}

pub fn print_graph<W: io::Write>(writer: &mut W, graph: &Graph) -> io::Result<()> {
    let arrow = match graph.gtype {
        GraphType::Directed => "->",
        GraphType::Undirected => "--",
    };
    let mut printer = Printer {
        arrow: arrow.to_string(),
        depth: 0,
        writer,
    };
    printer.visit_graph(graph)
}

struct Printer<W: io::Write> {
    arrow: String,
    depth: i32,
    writer: W,
}

impl<W: io::Write> Printer<W> {
    #[inline]
    fn indent(&mut self) -> io::Result<()> {
        for _ in 0..self.depth {
            self.writer.write_all(b"  ")?;
        }
        Ok(())
    }

    #[inline]
    fn arrow(&mut self, left: &str, right: &str) -> io::Result<()> {
        write!(self.writer, "\"{}\" {} \"{}\"", left, self.arrow, right)
    }

    fn print_attribute(&mut self, attribute: &Attribute) -> io::Result<()> {
        match attribute {
            Attribute::Color(c) => write!(self.writer, "color={}", c),
            Attribute::Label(l) => write!(self.writer, "label=\"{}\"", l),
            Attribute::Length(l) => write!(self.writer, "len={:.2}", l),
            Attribute::RankDir(rd) => write!(self.writer, "rankdir={}", rd),
            Attribute::Style(s) => write!(self.writer, "style={}", s),
            Attribute::Shape(s) => write!(self.writer, "shape={}", s),
            Attribute::Size(s) => write!(self.writer, "size={}", s),
        }
    }
}

impl<W: io::Write> AstVisitor<io::Result<()>> for Printer<W> {
    fn visit_graph(&mut self, graph: &Graph) -> io::Result<()> {
        if graph.strict {
            self.writer.write_all(b"strict ")?;
        }
        match graph.gtype {
            crate::ast::GraphType::Directed => write!(self.writer, "digraph")?,
            crate::ast::GraphType::Undirected => write!(self.writer, "graph")?,
        }

        if let Some(id) = &graph.id {
            write!(self.writer, " {}", id)?;
        }
        self.writer.write_all(b" {\n")?;
        self.depth += 1;
        for statement in graph.statements.iter() {
            self.visit_statement(statement)?;
        }
        self.depth -= 1;
        self.writer.write_all(b"}\n")?;
        Ok(())
    }

    fn visit_statement(&mut self, statement: &Statement) -> io::Result<()> {
        match statement {
            Statement::Attribute(AttributeScope::Default, attributes) => {
                for attribute in attributes {
                    self.indent()?;
                    self.print_attribute(attribute)?;
                    self.writer.write_all(b";\n")?;
                }
            }
            Statement::Attribute(scope, attributes) => {
                self.indent()?;
                match scope {
                    AttributeScope::Default => (),
                    AttributeScope::Graph => self.writer.write_all(b"graph")?,
                    AttributeScope::Node => self.writer.write_all(b"node")?,
                    AttributeScope::Edge => self.writer.write_all(b"edge")?,
                }
                self.visit_attributes(attributes)?;
                self.writer.write_all(b";\n")?;
            }
            Statement::Node { id, attributes } => {
                self.indent()?;
                write!(self.writer, "\"{}\"", id)?;
                self.visit_attributes(attributes)?;
                self.writer.write_all(b";\n")?;
            }
            Statement::Edge {
                from,
                to,
                attributes,
            } => {
                self.indent()?;
                self.arrow(from, to)?;
                self.visit_attributes(attributes)?;
                self.writer.write_all(b";\n")?;
            }
            Statement::Subgraph { id, statements } => {
                self.indent()?;
                self.writer.write_all(b"subgraph")?;
                if let Some(id) = id {
                    write!(self.writer, " {}", id)?;
                }
                self.writer.write_all(b" {\n")?;
                self.depth += 1;
                for statement in statements {
                    self.visit_statement(statement)?;
                }
                self.depth -= 1;
                self.indent()?;
                self.writer.write_all(b"}\n")?;
            }
        }
        Ok(())
    }

    fn visit_attributes(&mut self, attributes: &[Attribute]) -> io::Result<()> {
        match attributes {
            [] => {}
            [attribute] => {
                self.writer.write_all(b" [")?;
                self.print_attribute(attribute)?;
                self.writer.write_all(b"]")?;
            }
            [head, rest @ ..] => {
                self.writer.write_all(b" [")?;
                self.print_attribute(head)?;
                for attribute in rest {
                    self.writer.write_all(b", ")?;
                    self.print_attribute(attribute)?;
                }
                self.writer.write_all(b"]")?;
            }
        }
        Ok(())
    }
}
