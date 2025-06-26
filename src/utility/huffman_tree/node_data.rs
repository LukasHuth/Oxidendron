use crate::utility::huffman_tree::node::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeData {
    node: Node,
    pub(super) id: usize,
}
impl NodeData {
    pub fn new_leaf(value: u8, occurence: u64, node_collection: &mut Vec<Node>) -> Self {
        let leaf = Node::Leaf { value, occurence };
        node_collection.push(leaf);
        NodeData {
            node: leaf,
            id: node_collection.len() - 1,
        }
    }
    pub fn new_node(left: usize, right: usize, node_collection: &mut Vec<Node>) -> Self {
        let node = Node::Node {
            left,
            right,
            occurence: node_collection.get(left).map(Node::occurence).unwrap_or(0)
                + node_collection.get(right).map(Node::occurence).unwrap_or(0),
        };
        node_collection.push(node);
        NodeData {
            node,
            id: node_collection.len() - 1,
        }
    }
}
impl PartialOrd for NodeData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NodeData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.node.cmp(&other.node)
    }
}
