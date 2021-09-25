use graphe::attributes::{Color, EdgeStyle, NodeStyle, RankDir, Shape};
use graphe::{DirectedGraph, DotLayout, Graph, GraphBuilder, UnspecifiedOutput};
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn to_graph(&self) -> Graph {
        let mut builder = Builder::new();
        builder.visit_expr(self, None);
        builder.build()
    }
}

fn add(l: Expr, r: Expr) -> Expr {
    Expr::Add(Box::new(l), Box::new(r))
}

fn sub(l: Expr, r: Expr) -> Expr {
    Expr::Sub(Box::new(l), Box::new(r))
}

fn number(n: i64) -> Expr {
    Expr::Number(n)
}

struct Builder {
    builder: GraphBuilder<DirectedGraph, DotLayout, UnspecifiedOutput>,
    id_tracker: i64,
}

impl Builder {
    fn new() -> Self {
        let mut builder: GraphBuilder<DirectedGraph, DotLayout, UnspecifiedOutput> =
            graphe::directed().dot();
        builder.graph_attributes(|ab| ab.rankdir(RankDir::BottomTop));
        Self {
            builder,
            id_tracker: 0,
        }
    }

    fn next_id(&mut self) -> i64 {
        self.id_tracker += 1;
        self.id_tracker
    }

    fn build(self) -> Graph {
        self.builder.build()
    }

    fn visit_expr(&mut self, expr: &Expr, parent_id: Option<i64>) {
        let entry_id = self.next_id();
        match expr {
            Expr::Number(n) => {
                self.builder.node(entry_id.to_string().as_str(), |ab| {
                    ab.label(n.to_string().as_str())
                        .shape(Shape::Circle)
                        .style(NodeStyle::Filled)
                        .color(Color::LightGrey)
                });
                if let Some(id) = parent_id {
                    self.builder
                        .edge(entry_id.to_string().as_str(), &id.to_string(), |ab| {
                            ab.style(EdgeStyle::Dotted)
                        });
                }
            }
            Expr::Add(l, r) => {
                let entry_id = self.next_id();
                self.builder.node(entry_id.to_string().as_str(), |ab| {
                    ab.label("+").color(Color::Blue).shape(Shape::Box)
                });
                if let Some(id) = parent_id {
                    self.builder
                        .edge_(entry_id.to_string().as_str(), &id.to_string());
                }
                self.visit_expr(l, Some(entry_id));
                self.visit_expr(r, Some(entry_id));
            }
            Expr::Sub(l, r) => {
                let entry_id = self.next_id();
                self.builder.node(entry_id.to_string().as_str(), |ab| {
                    ab.label("-").color(Color::Red).shape(Shape::Diamond)
                });
                if let Some(id) = parent_id {
                    self.builder
                        .edge_(entry_id.to_string().as_str(), &id.to_string());
                }
                self.visit_expr(l, Some(entry_id));
                self.visit_expr(r, Some(entry_id));
            }
        }
    }
}

fn main() {
    let expr = add(
        add(number(1), number(2)),
        sub(number(3), add(number(4), number(5))),
    );
    let graph = expr.to_graph();
    let mut writer = Vec::new();
    graphe::print_graph(&mut writer, &graph).unwrap();
    let s = std::str::from_utf8(&writer).unwrap();
    println!("{}", s);
}
