use std::collections::BinaryHeap;

use crate::utility::{
    Occurences,
    huffman_tree::{node::Node, node_data::NodeData},
};

pub(super) mod node;
mod node_data;
mod tests;
#[derive(Debug)]
pub struct HuffmanTree {
    root: usize,
    pub(crate) nodes: Vec<Node>,
}
impl HuffmanTree {
    pub(super) fn root(&self) -> &Node {
        self.nodes
            .get(self.root)
            .expect("There should always be a root node")
    }
    pub fn find_occurences(input: &[u8]) -> Occurences {
        let mut occurences = [0; 256];
        for byte in input {
            occurences[*byte as usize] += 1;
        }
        occurences
    }
    pub fn generate(occurences: Occurences) -> Self {
        let mut nodes = BinaryHeap::new();
        let mut node_collection = Vec::new();
        for (value, occurence) in occurences.iter().enumerate().map(|(i, &v)| (i as u8, v)) {
            if occurence == 0 {
                continue;
            }
            nodes.push(NodeData::new_leaf(value, occurence, &mut node_collection));
        }
        if nodes.is_empty() {
            panic!("Got an empty occurence table");
        }
        while nodes.len() > 1 {
            let left = nodes
                .pop()
                .expect("This should never fail since the length check")
                .id;
            let right = nodes
                .pop()
                .expect("This should never fail since the length check")
                .id;
            nodes.push(NodeData::new_node(left, right, &mut node_collection));
        }
        let root = nodes.pop().expect("This should always return a node").id;
        HuffmanTree {
            root,
            nodes: node_collection,
        }
    }
}
