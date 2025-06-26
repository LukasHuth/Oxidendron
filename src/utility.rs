pub(crate) const CURRENT_VERSION: u8 = 1;
pub mod data_header;
pub mod decoding;
pub mod encoding;
pub mod huffman_tree;
pub(crate) type Occurences = [u64; 256];
