#[test]
pub fn test_huffman_tree_occurences() {
    use crate::utility::huffman_tree::HuffmanTree;
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
    use crate::utility::huffman_tree::HuffmanTree;
    let data = "Hello, World!";
    let occurences = HuffmanTree::find_occurences(data.as_bytes());
    let tree = HuffmanTree::generate(occurences);
    assert_eq!(tree.root, 18);
    assert_eq!(tree.root().occurence(), 13);
}
