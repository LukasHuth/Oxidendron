use std::{collections::HashMap, ops::Deref};

use crate::utility::huffman_tree::{HuffmanTree, node::Node};

#[derive(Debug, PartialEq, Eq)]
pub struct EncodingData {
    pub code: u8,
    pub length: u32,
}
pub struct EncodingTable {
    table: HashMap<u8, EncodingData>,
}
impl EncodingTable {
    pub fn new(tree: HuffmanTree) -> Self {
        let mut table = HashMap::new();
        // if there is only one thing there is no root node, so that has to be handled here
        // so the value does not have a length of 0
        Self::fill_map(
            tree.root(),
            &mut table,
            &tree,
            0,
            if tree.nodes.len() == 1 { 1 } else { 0 },
        );
        Self { table }
    }
    fn fill_map(
        node: &Node,
        table: &mut HashMap<u8, EncodingData>,
        tree: &HuffmanTree,
        code: u8,
        length: u32,
    ) {
        match node {
            Node::Leaf { value, .. } => {
                table.insert(*value, EncodingData { code, length });
            }
            Node::Node { left, right, .. } => {
                let left = tree
                    .nodes
                    .get(*left)
                    .expect("The referenced node should be available");
                let right = tree
                    .nodes
                    .get(*right)
                    .expect("The referenced node should be available");
                // Self::fill_map(left, table, tree, code << 1 | 1, length + 1);
                // Self::fill_map(right, table, tree, code << 1 | 0, length + 1);
                Self::fill_map(left, table, tree, code | 1 << length, length + 1);
                Self::fill_map(right, table, tree, code, length + 1);
            }
        }
    }
}
impl Deref for EncodingTable {
    type Target = HashMap<u8, EncodingData>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
#[test]
fn test_encoding_table() {
    let data = "Hello, World!";
    let occurences = HuffmanTree::find_occurences(data.as_bytes());
    let tree = HuffmanTree::generate(occurences);
    for (i, node) in tree.nodes.iter().enumerate() {
        match node {
            Node::Leaf { value, occurence } => println!("{i}: {value}({occurence})"),
            Node::Node {
                left,
                right,
                occurence,
            } => println!("{i}: {left},{right}({occurence})"),
        }
    }
    let encoding_table = EncodingTable::new(tree);
    let max = encoding_table.values().map(|e| e.length).max();
    assert!(matches!(max, Some(4)));
    let mut counts = [0; 5];
    for value in encoding_table.values() {
        counts[value.length as usize] += 1;
    }
    assert_eq!(counts[0], 0);
    assert_eq!(counts[1], 0);
    assert_eq!(counts[2], 1);
    assert_eq!(counts[3], 3);
    assert_eq!(counts[4], 6);
}
