use graphe::{DirectedGraph, DotLayout, Graph, GraphBuilder, UnspecifiedOutput};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PostgresPlan(Vec<Plan>);

#[derive(Debug, Deserialize)]
struct Plan {
    #[serde(rename(deserialize = "Planning Time"))]
    planning_time: f64,
    #[serde(rename(deserialize = "Plan"))]
    plan_node: PlanNode,
}

impl Plan {
    fn to_graph(&self) -> Graph {
        let mut builder = Builder::default();
        builder.visit_plan(self);
        builder.build()
    }
}

#[derive(Debug, Deserialize)]
struct PlanNode {
    #[serde(rename(deserialize = "Node Type"))]
    node_type: String,
    #[serde(rename(deserialize = "Plans"), default)]
    plans: Vec<PlanNode>,
}

struct Builder {
    builder: GraphBuilder<DirectedGraph, DotLayout, UnspecifiedOutput>,
    id_tracker: i64,
}

impl Builder {
    fn new() -> Self {
        let builder: GraphBuilder<DirectedGraph, DotLayout, UnspecifiedOutput> =
            graphe::directed().dot();
        Self {
            builder,
            id_tracker: 0,
        }
    }

    fn next_id(&mut self) -> i64 {
        self.id_tracker += 1;
        self.id_tracker
    }

    fn build(mut self) -> Graph {
        self.builder.build()
    }

    fn visit_plan(&mut self, plan: &Plan) {
        let entry_node = &plan.plan_node;
        let entry_id = self.next_id();
        self.builder
            .graph_attributes(|ab| ab.label("Postgres Plan"))
            .node(entry_id.to_string().as_str(), |ab| {
                ab.label(entry_node.node_type.as_str())
            });

        for plan_node in plan.plan_node.plans.iter() {
            self.visit_plan_node(plan_node, entry_id)
        }
    }

    fn visit_plan_node(&mut self, plan_node: &PlanNode, parent_id: i64) {
        let id = self.next_id();
        self.builder
            .node(id.to_string().as_str(), |ab| {
                ab.label(plan_node.node_type.as_str())
            })
            .edge_(parent_id.to_string().as_str(), id.to_string().as_str());
        for plan_node in plan_node.plans.iter() {
            self.visit_plan_node(plan_node, id)
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let pg_plan: PostgresPlan = serde_json::from_str(include_str!("pg_plan.json")).unwrap();
    let plan = &pg_plan.0[0];
    let graph = plan.to_graph();
    let mut writer = Vec::new();
    graphe::print_graph(&mut writer, &graph).unwrap();
    let s = std::str::from_utf8(&writer).unwrap();
    println!("{}", s)
}
