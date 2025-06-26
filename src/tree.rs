use std::{
    collections::{BinaryHeap, HashMap},
    io::Write,
    ops::Deref,
};
pub(crate) const CURRENT_VERSION: u8 = 1;
pub(crate) struct DataHeader {
    pub(crate) version: u8,
    pub(crate) occurences: Occurences,
    pub(crate) data_amount: u64,
}
impl DataHeader {
    pub(crate) const SIZE: usize =
        std::mem::size_of::<Occurences>() + std::mem::size_of::<u64>() + std::mem::size_of::<u8>();
    pub fn new(occurences: Occurences, data_amount: u64) -> Self {
        Self {
            version: CURRENT_VERSION,
            occurences,
            data_amount,
        }
    }
    pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CURRENT_VERSION])?;
        writer.write_all(self.occurences.map(u64::to_be_bytes).as_flattened())?;
        writer.write_all(&self.data_amount.to_be_bytes())
    }

    pub(crate) fn read_from(input: &[u8]) -> Self {
        let version = input[0];
        let occurences: Occurences = input[1..]
            .chunks_exact(std::mem::size_of::<u64>())
            .take(256)
            .map(|arr| u64::from_be_bytes(unsafe { arr.try_into().unwrap_unchecked() }))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        let data_amount = u64::from_be_bytes(
            input[Self::SIZE - 8..Self::SIZE]
                .to_vec()
                .try_into()
                .unwrap(),
        );
        Self {
            version,
            occurences,
            data_amount,
        }
    }
}
#[derive(Debug)]
pub struct HuffmanTree {
    root: usize,
    pub(crate) nodes: Vec<Node>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Node {
    Leaf {
        value: u8,
        occurence: u64,
    },
    Node {
        left: usize,
        right: usize,
        occurence: u64,
    },
}
impl Node {
    fn occurence(&self) -> u64 {
        match self {
            Node::Leaf { occurence, .. } | Node::Node { occurence, .. } => *occurence,
        }
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.occurence().cmp(&self.occurence()) {
            ord @ (std::cmp::Ordering::Less | std::cmp::Ordering::Greater) => ord,
            std::cmp::Ordering::Equal => match (self, other) {
                (Node::Node { .. }, Node::Leaf { .. }) => std::cmp::Ordering::Less,
                (Node::Leaf { .. }, Node::Node { .. }) => std::cmp::Ordering::Greater,
                (Node::Node { .. }, Node::Node { .. }) => std::cmp::Ordering::Equal,
                (Node::Leaf { value: v0, .. }, Node::Leaf { value: v1, .. }) => v1.cmp(v0),
            },
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeData {
    node: Node,
    id: usize,
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
pub(crate) type Occurences = [u64; 256];
impl HuffmanTree {
    fn root(&self) -> &Node {
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
#[test]
pub fn test_huffman_tree_occurences() {
    let data = "Hello, World!";
    let mut expected = [0; 256];
    expected['H' as usize] += 1;
    expected['e' as usize] += 1;
    expected['l' as usize] += 3;
    expected['o' as usize] += 2;
    expected[',' as usize] += 1;
    expected[' ' as usize] += 1;
    expected['W' as usize] += 1;
    expected['r' as usize] += 1;
    expected['d' as usize] += 1;
    expected['!' as usize] += 1;
    assert_eq!(expected, HuffmanTree::find_occurences(data.as_bytes()));
}
#[test]
pub fn test_huffman_tree() {
    let data = "Hello, World!";
    let occurences = HuffmanTree::find_occurences(data.as_bytes());
    let tree = HuffmanTree::generate(occurences);
    assert_eq!(tree.root, 18);
    assert_eq!(tree.root().occurence(), 13);
    assert_eq!(tree.root().right(&tree.nodes).occurence(), 8);
    assert_eq!(tree.root().left(&tree.nodes).occurence(), 5);
}
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
