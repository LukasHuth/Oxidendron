use std::{collections::HashMap, ops::Deref};

use crate::utility::huffman_tree::{HuffmanTree, node::Node};

#[derive(Debug)]
pub struct DecodeTable {
    table: HashMap<(u8, usize), u8>,
    pub(crate) max_length: usize,
}
impl Deref for DecodeTable {
    type Target = HashMap<(u8, usize), u8>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
impl DecodeTable {
    pub fn new(tree: HuffmanTree) -> Self {
        let mut table = HashMap::new();
        let max_length = Self::fill_map(
            tree.root(),
            &mut table,
            &tree,
            0,
            if tree.nodes.len() == 1 { 1 } else { 0 },
            0,
        ) as usize;
        Self { table, max_length }
    }
    fn fill_map(
        node: &Node,
        table: &mut HashMap<(u8, usize), u8>,
        tree: &HuffmanTree,
        code: u8,
        length: u32,
        max_length: u32,
    ) -> u32 {
        match node {
            Node::Leaf { value, .. } => {
                table.insert((code, length as usize), *value);
                if max_length < length {
                    length
                } else {
                    max_length
                }
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
                let max_length = Self::fill_map(
                    left,
                    table,
                    tree,
                    code | 1 << length,
                    length + 1,
                    max_length,
                );
                Self::fill_map(right, table, tree, code, length + 1, max_length)
                // let max_length =
                //     Self::fill_map(left, table, tree, code << 1 | 1 , length + 1, max_length);
                // Self::fill_map(right, table, tree, code << 1, length + 1, max_length)
            }
        }
    }
}
