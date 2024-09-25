use std::cell::RefCell;
use std::rc::{Rc, Weak};

enum SupplyType {
    Core(String),
    Neutral,
}

enum LandType {
    Normal,
    SupplyCenter(SupplyType),
}

enum TerritoryType {
    Sea,
    Land(LandType),
}

struct Node {
    name: String,
    attributes: TerritoryType,
    neighbors: Vec<NodeWeakRef>,
}

impl Node {
    fn new(name: String, attributes: TerritoryType) -> Self {
        Node{
            name,
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
    fn add_node(&mut self, node: Node) -> NodeRef {
        let node = Rc::new(RefCell::new(node));
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

fn gen_test_turkey() -> Graph {
    let mut g = Graph::new();

    let con = Node::new(
        "Constantinople".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Turkey".to_string()))
        )
    );

    let ank = Node::new(
        "Ankara".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Turkey".to_string()))
        )
    );

    let smy = Node::new(
        "Smyrna".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Turkey".to_string()))
        )
    );

    let con = g.add_node(con);
    let smy = g.add_node(smy);
    let ank = g.add_node(ank);

    g.add_edge(&con, &ank);
    g.add_edge(&con, &smy);
    g.add_edge(&ank, &smy);

    g
}

fn gen_test_turkey_region() -> Graph {
    let mut g = Graph::new();

    let con = Node::new(
        "Constantinople".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Turkey".to_string()))
        )
    );

    let ank = Node::new(
        "Ankara".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Turkey".to_string()))
        )
    );

    let smy = Node::new(
        "Smyrna".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Turkey".to_string()))
        )
    );

    let sev = Node::new(
        "Sevastopol".to_string(),
        TerritoryType::Land(
            LandType::SupplyCenter(
                SupplyType::Core("Russia".to_string()))
        )
    );

    let bla = Node::new(
        "Black Sea".to_string(),
        TerritoryType::Sea
    );

    let eas = Node::new(
        "Eastern Mediterranean".to_string(),
        TerritoryType::Sea
    );

    let arm = Node::new(
        "Armenia".to_string(),
        TerritoryType::Land(LandType::Normal)
    );

    let syr = Node::new(
        "Syria".to_string(),
        TerritoryType::Land(LandType::Normal)
    );

    let con = g.add_node(con);
    let smy = g.add_node(smy);
    let ank = g.add_node(ank);
    let sev = g.add_node(sev);
    let bla = g.add_node(bla);
    let eas = g.add_node(eas);
    let arm = g.add_node(arm);
    let syr = g.add_node(syr);

    g.add_edge(&con, &ank);
    g.add_edge(&con, &smy);
    g.add_edge(&con, &bla);

    g.add_edge(&ank, &bla);
    g.add_edge(&ank, &smy);
    g.add_edge(&ank, &arm);

    g.add_edge(&smy, &eas);
    g.add_edge(&smy, &arm);
    g.add_edge(&smy, &syr);

    g.add_edge(&sev, &bla);
    g.add_edge(&sev, &arm);

    g.add_edge(&eas, &syr);
    g.add_edge(&bla, &arm);
    g.add_edge(&arm, &syr);

    g
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_turkey_nodes() {
        let turkey = gen_test_turkey();

        // Build a map from node names to NodeRefs for easy access
        let mut node_map: HashMap<String, NodeRef> = HashMap::new();
        for node_ref in &turkey.nodes {
            let node = node_ref.borrow();
            node_map.insert(node.name.clone(), Rc::clone(node_ref));
        }

        // Check that all expected nodes are present
        let expected_nodes = vec![
            "Constantinople",
            "Ankara",
            "Smyrna",
        ];
        assert_eq!(node_map.len(), expected_nodes.len());

        for name in &expected_nodes {
            assert!(
                node_map.contains_key(*name),
                "Node '{}' is missing from the graph",
                name
            );
        }
    }
}
