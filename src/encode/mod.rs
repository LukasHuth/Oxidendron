use std::{
    fmt::Debug,
    hint::unreachable_unchecked,
    io::{BufWriter, Write},
};

use crate::{
    Huffman,
    utility::{
        data_header::DataHeader,
        encoding::{EncodingData, EncodingTable},
        huffman_tree::HuffmanTree,
    },
};

impl Huffman {
    pub fn encode<T>(input: &[u8], output: &mut BufWriter<T>)
    where
        T: Write + Debug,
    {
        let occurences = HuffmanTree::find_occurences(input);
        let tree = HuffmanTree::generate(occurences);
        // for (i, node) in tree.nodes.iter().enumerate() {
        //     match node {
        //         Node::Leaf { value, occurence } => println!("{i}: {value}({occurence})"),
        //         Node::Node {
        //             left,
        //             right,
        //             occurence,
        //         } => println!("{i}: {left},{right}({occurence})"),
        //     }
        // }
        let encoding_table = EncodingTable::new(tree);
        // Debug data
        // for (value, encoding_data) in encoding_table.iter() {
        //     println!("{value} {:0b},{}", encoding_data.code, encoding_data.length);
        // }

        // Write data header
        let data_header = DataHeader::new(occurences, input.len() as u64);
        data_header.write_to(output).unwrap();
        // println!("{output:?}");

        let mut state = EncodingState::new();
        for byte in input {
            encode(*byte, output, &encoding_table, &mut state);
        }
        if state.offset != 0 {
            output.write_all(&[state.current_byte]).unwrap();
        }
        output.flush().unwrap();
        // println!("{output:?}");
    }
}
struct EncodingState {
    current_byte: u8,
    offset: u32,
}
impl EncodingState {
    pub fn new() -> Self {
        Self {
            current_byte: 0,
            offset: 0,
        }
    }
}
fn encode<T>(
    input: u8,
    output: &mut BufWriter<T>,
    encoding_table: &EncodingTable,
    state: &mut EncodingState,
) where
    T: Write,
{
    let encoding_data = encoding_table.get(&input).expect("Missing an entry");
    write(encoding_data, output, state);
}
fn write<T>(input: &EncodingData, output: &mut BufWriter<T>, state: &mut EncodingState)
where
    T: Write,
{
    match state.offset + input.length {
        0 => unsafe { unreachable_unchecked() },
        1..=7 => {
            // println!("normal insert");
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
            state.current_byte |= input.code << state.offset;
            state.offset += input.length;
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
        }
        8 => {
            // println!("Finished byte");
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
            state.current_byte |= input.code << state.offset;
            state.offset = 0;
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
            output.write_all(&[state.current_byte]).unwrap();
            state.current_byte = 0;
        }
        9.. => {
            // println!("Overflow insert");
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
            state.current_byte |= input.code.wrapping_shl(state.offset);
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
            output.write_all(&[state.current_byte]).unwrap();
            let missing_bits = (state.offset + input.length) - 8;
            state.current_byte =
                (input.code.wrapping_shr(input.length - missing_bits)) & ((1 << missing_bits) - 1);
            state.offset = missing_bits;
            // println!(
            //     "{:b}({})    {:b}({})",
            //     state.current_byte, state.offset, input.code, input.length
            // );
            // println!("===")
        }
    }
}
#[test]
fn test_encoding() {
    let data = "Hello, World!".as_bytes();
    let mut result = Vec::new();
    Huffman::encode(data, &mut BufWriter::new(&mut result));
    assert!(!result.is_empty());
    /* Data
     * H    e    l  l  o   ,    spc  W   o   r    l  d   !
     * 0000 0101 10 10 011 0001 0011 111 011 0100 10 110 0010
     *
     * bytes (lsb first):
     * 0101_0000 1_011_10_10 1_0011_000 100_011_11 10_110_10_0 ??????_00
     */
    // const OFFSET: usize = DataHeader::SIZE;
    let offset: usize = result.len() - 6;
    assert_eq!(result[offset + 0], 0b1010_0000);
    assert_eq!(result[offset + 1], 0b0_110_01_01);
    assert_eq!(result[offset + 2], 0b1_1100_100);
    assert_eq!(result[offset + 3], 0b010_110_11);
    assert_eq!(result[offset + 4], 0b00_011_01_0);
    assert_eq!(result[offset + 5], 0b01);
}
