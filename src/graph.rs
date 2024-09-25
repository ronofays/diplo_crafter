use std::cell::RefCell;
use std::rc::{Rc, Weak};

enum SupplyType {
    Core(String),
    Neutral,
}

enum LandType {
    Normal,
    SupplyCenter,
}

enum TerritoryType {
    Sea,
    Land(LandType),
}

struct Node {
    attributes: TerritoryType,
    neighbors: Vec<NodeWeakRef>,
}

impl Node {
    fn new(attributes: TerritoryType) -> Self {
        Node{
            attributes,
            neighbors: Vec::new(),
        }
    }
}

type NodeRef = Rc<RefCell<Node>>;
type NodeWeakRef = Weak<RefCell<Node>>;

struct Graph {
    nodes: Vec<NodeRef>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
}

impl Graph {
    // Add a new node to the graph and return a strong reference to it
    fn add_node(&mut self, attributes: TerritoryType) -> NodeRef {
        let node = Rc::new(RefCell::new(Node::new(attributes)));
        self.nodes.push(node.clone());
        node
    }

    // Remove a node from the graph
    fn remove_node(&mut self, node_ref: &NodeRef) {
        // Remove the node from the graph's node list
        self.nodes.retain(|n| !Rc::ptr_eq(n, node_ref));

        // Remove the node from neighbors' lists
        for node in &self.nodes {
            node.borrow_mut().neighbors.retain(|neighbor_weak| {
                neighbor_weak.upgrade().map_or(false, |neighbor_strong| {
                    !Rc::ptr_eq(&neighbor_strong, node_ref)
                })
            });
        }
    }

    // Add an edge between two nodes
    fn add_edge(&self, node1: &NodeRef, node2: &NodeRef) {
        node1.borrow_mut().neighbors.push(Rc::downgrade(node2));
        node2.borrow_mut().neighbors.push(Rc::downgrade(node1)); // For undirected graphs
    }

    // Remove an edge between two nodes
    fn remove_edge(&self, node1: &NodeRef, node2: &NodeRef) {
        node1.borrow_mut().neighbors.retain(|neighbor_weak| {
            neighbor_weak.upgrade().map_or(false, |neighbor_strong| {
                !Rc::ptr_eq(&neighbor_strong, node2)
            })
        });
        node2.borrow_mut().neighbors.retain(|neighbor_weak| {
            neighbor_weak.upgrade().map_or(false, |neighbor_strong| {
                !Rc::ptr_eq(&neighbor_strong, node1)
            })
        });
    }
}
