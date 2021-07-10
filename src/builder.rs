#![allow(dead_code)]

use std::marker::PhantomData;

use crate::ast;
use crate::ast::*;

pub struct DirectedGraph;
pub struct UndirectedGraph;

pub trait GraphType {}
impl GraphType for DirectedGraph {}
impl GraphType for UndirectedGraph {}

pub struct GraphContext;
pub struct NodeContext;
pub struct EdgeContext;
pub struct SubgraphContext;
pub struct ClusterContext;
pub struct DefaultContext;

pub trait EntityContext {}
impl EntityContext for GraphContext {}
impl EntityContext for NodeContext {}
impl EntityContext for EdgeContext {}
impl EntityContext for SubgraphContext {}
impl EntityContext for ClusterContext {}
impl EntityContext for DefaultContext {}

pub struct DotLayout;
pub struct NeatoLayout;
pub struct UnspecifiedLayout;

pub trait LayoutContext {}
impl LayoutContext for DotLayout {}
impl LayoutContext for NeatoLayout {}
impl LayoutContext for UnspecifiedLayout {}

pub struct BitmapOutput;
pub struct PostscriptOutput;
pub struct SVGOutput;
pub struct UnspecifiedOutput;

pub trait OutputContext {}
impl OutputContext for BitmapOutput {}
impl OutputContext for PostscriptOutput {}
impl OutputContext for SVGOutput {}
impl OutputContext for UnspecifiedOutput {}

pub fn directed() -> GraphBuilder<DirectedGraph, UnspecifiedLayout, UnspecifiedOutput> {
    GraphBuilder {
        statements: Vec::new(),
        graph_type: PhantomData,
        layout_context: PhantomData,
        output_context: PhantomData,
    }
}

pub fn undirected() -> GraphBuilder<UndirectedGraph, UnspecifiedLayout, UnspecifiedOutput> {
    GraphBuilder {
        statements: Vec::new(),
        graph_type: PhantomData,
        layout_context: PhantomData,
        output_context: PhantomData,
    }
}

pub struct GraphBuilder<GT: GraphType, LC: LayoutContext, OC: OutputContext> {
    statements: Vec<Statement>,
    graph_type: PhantomData<GT>,
    layout_context: PhantomData<LC>,
    output_context: PhantomData<OC>,
}

impl<LC, OC> GraphBuilder<DirectedGraph, LC, OC>
where
    LC: LayoutContext,
    OC: OutputContext,
{
    pub fn build(self) -> Graph {
        Graph {
            strict: false,
            gtype: ast::GraphType::Directed,
            id: None,
            statements: self.statements,
        }
    }
}

impl<LC, OC> GraphBuilder<UndirectedGraph, LC, OC>
where
    LC: LayoutContext,
    OC: OutputContext,
{
    pub fn build(self) -> Graph {
        Graph {
            strict: false,
            gtype: ast::GraphType::Undirected,
            id: None,
            statements: self.statements,
        }
    }
}

impl<GT, LC, OC> GraphBuilder<GT, LC, OC>
where
    GT: GraphType,
    LC: LayoutContext,
    OC: OutputContext,
{
    pub fn attributes<F>(&mut self, f: F) -> &mut GraphBuilder<GT, LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<DefaultContext, LC, OC>,
        ) -> &mut AttributeBuilder<DefaultContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<DefaultContext, LC, OC> =
            AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Attribute(
            AttributeScope::Default,
            attribute_builder.build(),
        ));
        self
    }

    fn node_attributes<F>(&mut self, f: F) -> &mut GraphBuilder<GT, LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<NodeContext, LC, OC>,
        ) -> &mut AttributeBuilder<NodeContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<NodeContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Attribute(
            AttributeScope::Node,
            attribute_builder.build(),
        ));
        self
    }

    fn graph_attributes<F>(&mut self, f: F) -> &mut GraphBuilder<GT, LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<GraphContext, LC, OC>,
        ) -> &mut AttributeBuilder<GraphContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<GraphContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Attribute(
            AttributeScope::Graph,
            attribute_builder.build(),
        ));
        self
    }

    pub fn node<F>(&mut self, id: &str, f: F) -> &mut GraphBuilder<GT, LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<NodeContext, LC, OC>,
        ) -> &mut AttributeBuilder<NodeContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<NodeContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Node {
            id: id.to_string(),
            attributes: attribute_builder.build(),
        });
        self
    }

    pub fn node_(&mut self, id: &str) -> &mut GraphBuilder<GT, LC, OC> {
        self.statements.push(Statement::Node {
            id: id.to_string(),
            attributes: Vec::new(),
        });
        self
    }

    pub fn edge<F>(&mut self, from: &str, to: &str, f: F) -> &mut GraphBuilder<GT, LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<EdgeContext, LC, OC>,
        ) -> &mut AttributeBuilder<EdgeContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<EdgeContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Edge {
            from: from.to_string(),
            to: to.to_string(),
            attributes: attribute_builder.build(),
        });
        self
    }

    pub fn edge_(&mut self, from: &str, to: &str) -> &mut GraphBuilder<GT, LC, OC> {
        self.statements.push(Statement::Edge {
            from: from.to_string(),
            to: to.to_string(),
            attributes: Vec::new(),
        });
        self
    }

    pub fn cluster<F>(&mut self, id: &str, f: F) -> &mut GraphBuilder<GT, LC, OC>
    where
        F: FnOnce(&mut StatementBuilder<LC, OC>) -> &mut StatementBuilder<LC, OC>,
    {
        let mut statement_builder: StatementBuilder<LC, OC> = StatementBuilder::new();
        f(&mut statement_builder);

        self.statements.push(Statement::Subgraph {
            id: Some(format!("cluster_{}", id)),
            statements: statement_builder.build(),
        });
        self
    }
}

impl<GT, LC> GraphBuilder<GT, LC, UnspecifiedOutput>
where
    GT: GraphType,
    LC: LayoutContext,
{
    pub fn bitmap(self) -> GraphBuilder<GT, LC, BitmapOutput> {
        GraphBuilder {
            statements: self.statements,
            graph_type: self.graph_type,
            layout_context: self.layout_context,
            output_context: PhantomData,
        }
    }

    pub fn svg(self) -> GraphBuilder<GT, LC, SVGOutput> {
        GraphBuilder {
            statements: self.statements,
            graph_type: self.graph_type,
            layout_context: self.layout_context,
            output_context: PhantomData,
        }
    }
}

impl<GT, OC> GraphBuilder<GT, UnspecifiedLayout, OC>
where
    GT: GraphType,
    OC: OutputContext,
{
    pub fn dot(self) -> GraphBuilder<GT, DotLayout, OC> {
        GraphBuilder {
            statements: self.statements,
            graph_type: self.graph_type,
            layout_context: PhantomData,
            output_context: self.output_context,
        }
    }

    pub fn neato(self) -> GraphBuilder<GT, NeatoLayout, OC> {
        GraphBuilder {
            statements: self.statements,
            graph_type: self.graph_type,
            layout_context: PhantomData,
            output_context: self.output_context,
        }
    }
}

pub struct StatementBuilder<LC: LayoutContext, OC: OutputContext> {
    statements: Vec<Statement>,
    layout_context: PhantomData<LC>,
    output_context: PhantomData<OC>,
}

impl<LC, OC> StatementBuilder<LC, OC>
where
    LC: LayoutContext,
    OC: OutputContext,
{
    fn new() -> StatementBuilder<LC, OC> {
        StatementBuilder {
            statements: Vec::new(),
            layout_context: PhantomData,
            output_context: PhantomData,
        }
    }

    fn build(self) -> Vec<Statement> {
        self.statements
    }

    pub fn attributes<F>(&mut self, f: F) -> &mut StatementBuilder<LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<DefaultContext, LC, OC>,
        ) -> &mut AttributeBuilder<DefaultContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<DefaultContext, LC, OC> =
            AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Attribute(
            AttributeScope::Default,
            attribute_builder.build(),
        ));
        self
    }

    fn node_attributes<F>(&mut self, f: F) -> &mut StatementBuilder<LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<NodeContext, LC, OC>,
        ) -> &mut AttributeBuilder<NodeContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<NodeContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Attribute(
            AttributeScope::Node,
            attribute_builder.build(),
        ));
        self
    }

    pub fn node<F>(&mut self, id: &str, f: F) -> &mut StatementBuilder<LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<NodeContext, LC, OC>,
        ) -> &mut AttributeBuilder<NodeContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<NodeContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Node {
            id: id.to_string(),
            attributes: attribute_builder.build(),
        });
        self
    }

    pub fn edge<F>(&mut self, from: &str, to: &str, f: F) -> &mut StatementBuilder<LC, OC>
    where
        F: FnOnce(
            &mut AttributeBuilder<EdgeContext, LC, OC>,
        ) -> &mut AttributeBuilder<EdgeContext, LC, OC>,
    {
        let mut attribute_builder: AttributeBuilder<EdgeContext, LC, OC> = AttributeBuilder::new();
        f(&mut attribute_builder);
        self.statements.push(Statement::Edge {
            from: from.to_string(),
            to: to.to_string(),
            attributes: attribute_builder.build(),
        });
        self
    }

    pub fn edge_(&mut self, from: &str, to: &str) -> &mut StatementBuilder<LC, OC> {
        self.statements.push(Statement::Edge {
            from: from.to_string(),
            to: to.to_string(),
            attributes: Vec::new(),
        });
        self
    }
}

pub struct AttributeBuilder<EC: EntityContext, LC: LayoutContext, OC: OutputContext> {
    attributes: Vec<Attribute>,
    entity_context: PhantomData<EC>,
    layout_context: PhantomData<LC>,
    output_context: PhantomData<OC>,
}

impl<EC, LC, OC> AttributeBuilder<EC, LC, OC>
where
    EC: EntityContext,
    LC: LayoutContext,
    OC: OutputContext,
{
    fn new() -> AttributeBuilder<EC, LC, OC> {
        AttributeBuilder {
            attributes: Vec::new(),
            entity_context: PhantomData,
            layout_context: PhantomData,
            output_context: PhantomData,
        }
    }

    fn build(self) -> Vec<Attribute> {
        self.attributes
    }

    pub fn label(&mut self, label: &str) -> &mut AttributeBuilder<EC, LC, OC> {
        self.attributes.push(Attribute::Label(label.to_string()));
        self
    }
}

impl<LC, OC> AttributeBuilder<GraphContext, LC, OC>
where
    LC: LayoutContext,
    OC: OutputContext,
{
    fn size(&mut self, size: Size) -> &mut AttributeBuilder<GraphContext, LC, OC> {
        self.attributes.push(Attribute::Size(size));
        self
    }
}

impl<OC> AttributeBuilder<GraphContext, DotLayout, OC>
where
    OC: OutputContext,
{
    fn rankdir(&mut self, rankdir: RankDir) -> &mut AttributeBuilder<GraphContext, DotLayout, OC> {
        self.attributes.push(Attribute::RankDir(rankdir));
        self
    }
}

impl<LC, OC> AttributeBuilder<NodeContext, LC, OC>
where
    LC: LayoutContext,
    OC: OutputContext,
{
    fn color(&mut self, color: Color) -> &mut AttributeBuilder<NodeContext, LC, OC> {
        self.attributes.push(Attribute::Color(color));
        self
    }
    fn shape(&mut self, shape: Shape) -> &mut AttributeBuilder<NodeContext, LC, OC> {
        self.attributes.push(Attribute::Shape(shape));
        self
    }

    fn style(&mut self, style: NodeStyle) -> &mut AttributeBuilder<NodeContext, LC, OC> {
        self.attributes.push(Attribute::StyleNode(style));
        self
    }
}

impl<LC, OC> AttributeBuilder<DefaultContext, LC, OC>
where
    LC: LayoutContext,
    OC: OutputContext,
{
    fn color(&mut self, color: Color) -> &mut AttributeBuilder<DefaultContext, LC, OC> {
        self.attributes.push(Attribute::Color(color));
        self
    }

    fn shape(&mut self, shape: Shape) -> &mut AttributeBuilder<DefaultContext, LC, OC> {
        self.attributes.push(Attribute::Shape(shape));
        self
    }

    // TODO: In theory, this should be able to take ANY style.
    fn style(&mut self, style: NodeStyle) -> &mut AttributeBuilder<DefaultContext, LC, OC> {
        self.attributes.push(Attribute::StyleNode(style));
        self
    }
}

impl<OC> AttributeBuilder<DefaultContext, DotLayout, OC>
where
    OC: OutputContext,
{
    fn rankdir(
        &mut self,
        rankdir: RankDir,
    ) -> &mut AttributeBuilder<DefaultContext, DotLayout, OC> {
        self.attributes.push(Attribute::RankDir(rankdir));
        self
    }
}

impl<OC> AttributeBuilder<EdgeContext, NeatoLayout, OC>
where
    OC: OutputContext,
{
    fn len(&mut self, length: f64) -> &mut AttributeBuilder<EdgeContext, NeatoLayout, OC> {
        self.attributes.push(Attribute::Length(length));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printer::print_graph;
    use std::str;

    #[test]
    fn hello_world() {
        // https://graphviz.org/Gallery/directed/hello.html
        let mut builder = directed().dot();
        builder.edge_("Hello", "World");

        let graph = builder.build();
        let mut writer = Vec::new();
        print_graph(&mut writer, &graph).unwrap();
        let s = str::from_utf8(&writer).unwrap();
        println!("{}", s);
    }

    #[test]
    fn clusters() {
        // https://graphviz.org/Gallery/directed/cluster.html{
        let mut builder = directed().dot();
        builder
            .cluster("0", |builder| {
                builder
                    .attributes(|builder| builder.style(NodeStyle::Filled).color(Color::LightGrey))
                    .node_attributes(|builder| builder.style(NodeStyle::Filled).color(Color::White))
                    .edge_("a0", "a1")
                    .edge_("a1", "a2")
                    .edge_("a2", "a3")
                    .attributes(|builder| builder.label("process #1"))
            })
            .cluster("1", |builder| {
                builder
                    .node_attributes(|builder| builder.style(NodeStyle::Filled))
                    .edge_("b0", "b1")
                    .edge_("b1", "b2")
                    .edge_("b2", "b3")
                    .attributes(|builder| builder.label("process #2").color(Color::Blue))
            })
            .edge_("start", "a0")
            .edge_("start", "b0")
            .edge_("a1", "b3")
            .edge_("b2", "a3")
            .edge_("a3", "a0")
            .edge_("a3", "end")
            .edge_("b3", "end")
            .node("start", |builder| builder.shape(Shape::MDiamond))
            .node("end", |builder| builder.shape(Shape::MSquare));

        let graph = builder.build();
        let mut writer = Vec::new();
        print_graph(&mut writer, &graph).unwrap();
        let s = str::from_utf8(&writer).unwrap();
        println!("{}", s);
    }

    #[test]
    fn finite_state_machine() {
        // https://graphviz.org/Gallery/directed/fsm.html
        let mut builder = directed().dot();
        builder
            .attributes(|builder| builder.rankdir(RankDir::LeftRight))
            .graph_attributes(|builder| builder.size((8., 5.).into()))
            .node_attributes(|builder| builder.shape(Shape::DoubleCircle))
            .node_("0")
            .node_("3")
            .node_("4")
            .node_("8")
            .node_attributes(|builder| builder.shape(Shape::Circle))
            .edge("0", "2", |builder| builder.label("SS(B)"))
            .edge("0", "1", |builder| builder.label("SS(B)"))
            .edge("1", "3", |builder| builder.label("SS(B)"));

        let graph = builder.build();
        let mut writer = Vec::new();
        print_graph(&mut writer, &graph).unwrap();
        let s = str::from_utf8(&writer).unwrap();
        println!("{}", s)
    }

    #[test]
    fn entity_relation_diagram() {
        // https://graphviz.org/Gallery/undirected/ER.html
        let mut builder = undirected().neato();
        builder
            .node("course", |ab| ab.shape(Shape::Box))
            .node("institute", |ab| ab.shape(Shape::Box))
            .node("student", |ab| ab.shape(Shape::Box))
            .node("name0", |ab| ab.shape(Shape::Ellipse).label("name"))
            .node("name1", |ab| ab.shape(Shape::Ellipse).label("name"))
            .node("name2", |ab| ab.shape(Shape::Ellipse).label("name"))
            .node("code", |ab| ab.shape(Shape::Ellipse))
            .node("grade", |ab| ab.shape(Shape::Ellipse))
            .node("number", |ab| ab.shape(Shape::Ellipse))
            .node("C-I", |ab| ab.shape(Shape::Diamond).color(Color::LightGrey))
            .node("S-C", |ab| ab.shape(Shape::Diamond).color(Color::LightGrey))
            .node("S-I", |ab| ab.shape(Shape::Diamond).color(Color::LightGrey))
            .edge_("name0", "course")
            .edge_("code", "course")
            .edge("course", "C-I", |ab| ab.label("n").len(1.00))
            .edge("C-I", "institute", |ab| ab.label("1").len(1.00))
            .edge_("institute", "name1")
            .edge("institute", "S-I", |ab| ab.label("1").len(1.00))
            .edge("S-I", "student", |ab| ab.label("n").len(1.00))
            .edge_("student", "grade")
            .edge_("student", "name2")
            .edge_("student", "number")
            .edge("student", "S-C", |ab| ab.label("m").len(1.00))
            .edge("S-C", "course", |ab| ab.label("n").len(1.00));

        let graph = builder.build();
        let mut writer = Vec::new();
        print_graph(&mut writer, &graph).unwrap();
        let s = str::from_utf8(&writer).unwrap();
        println!("{}", s)
    }
}
