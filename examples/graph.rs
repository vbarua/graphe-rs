use graphe::{directed, print_graph, Graph};

type NodeId<'a> = &'a str;
struct Node<'a> {
    id: NodeId<'a>,
    name: &'a str,
}

type AdjacencyList<'a> = Vec<(Node<'a>, Vec<NodeId<'a>>)>;

fn to_graph(aj: &AdjacencyList) -> Graph {
    let mut builder = directed().dot();
    for (node, edges) in aj {
        builder.node(node.id, |b| b.label(node.name));
        for edge in edges {
            builder.edge_(node.id, edge);
        }
    }
    builder.build()
}

fn main() {
    let a = Node {
        id: "a",
        name: "Start",
    };
    let b = Node {
        id: "b",
        name: "Step 1",
    };
    let c = Node {
        id: "c",
        name: "Step 2",
    };
    let d = Node {
        id: "d",
        name: "Step 3",
    };
    let e = Node {
        id: "e",
        name: "End",
    };

    let aj: AdjacencyList = vec![
        (a, vec!["b"]),
        (b, vec!["c", "d"]),
        (c, vec!["d"]),
        (d, vec!["d", "e"]),
        (e, vec![]),
    ];

    let graph = to_graph(&aj);

    let mut writer = Vec::new();
    print_graph(&mut writer, &graph).unwrap();
    let s = std::str::from_utf8(&writer).unwrap();
    println!("{}", s)
}
