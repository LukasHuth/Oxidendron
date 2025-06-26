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
    pub(super) fn occurence(&self) -> u64 {
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
