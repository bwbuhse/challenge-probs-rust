use std::collections::HashMap;

struct Node {
    n: i32,
    edges: Vec<Edge>,
}

#[derive(Clone)]
struct Edge {
    // from dislikes to
    from: i32,
    to: i32,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Node {
    const fn new(n: i32) -> Node {
        Node {
            n: n,
            edges: Vec::new(),
        }
    }
}

impl Edge {
    const fn new(from: i32, to: i32) -> Edge {
        Edge { from: from, to: to }
    }
}

impl Graph {
    fn new(n: i32, dislikes: Vec<Vec<i32>>) -> Graph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_set: HashMap<i32, Node> = HashMap::new();

        for dislike_pair in dislikes {
            let i = dislike_pair[0];
            let j = dislike_pair[1];
            let i_node;
            let j_node;

            if let Some(node) = node_set.get_mut(&i) {
                i_node = node;
            } else {
                let node = Node::new(i);
                node_set.insert(i, node);
                i_node = node_set.get_mut(&i).unwrap();
            }

            if let Some(node) = node_set.get_mut(&j) {
                j_node = node;
            } else {
                let node = Node::new(j);
                node_set.insert(j, node);
                j_node = &mut node;
            }

            let ij_edge = Edge::new(i, j);
            i_node.edges.push(ij_edge.clone());
            j_node.edges.push(ij_edge.clone());
        }

        // for i in 1..=n {
        //     let mut node = Node::new(i);

        //     // Create the edges by filtering to only dislikes,
        //     // everything else can be an edge
        //     nums.clone()
        //         .filter(|x| *x != i && dislikes[i as usize].contains(x))
        //         .for_each(|x| {
        //             let edge = Edge::new(i, x);
        //             edges.push(edge.clone());
        //             node.edges.push(edge);
        //         });

        //     nodes.push(node);
        // }

        Graph {
            nodes: nodes,
            edges: edges,
        }
    }

    fn possible_bipartition(&self) -> bool {
        false
    }
}

pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let graph = Graph::new(n, dislikes);
    graph.possible_bipartition()
}
