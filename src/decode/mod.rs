use std::{
    fmt::Debug,
    io::{BufWriter, Write},
};

use byteorder::WriteBytesExt;

use crate::{
    Huffman,
    utility::{
        CURRENT_VERSION, data_header::DataHeader, decoding::DecodeTable, huffman_tree::HuffmanTree,
    },
};

impl Huffman {
    pub fn decode<T>(input: &[u8], output: &mut BufWriter<T>)
    where
        T: Write + Debug,
    {
        let data_header = DataHeader::read_from(input);
        if data_header.version != CURRENT_VERSION {
            panic!(
                "File was created in an earlier version. Parsing of older versions is currently not supported"
            );
        }
        let tree = HuffmanTree::generate(data_header.occurences);
        let lookup_table = DecodeTable::new(tree);
        let input = &input[DataHeader::SIZE..];
        let mut offset = 0;
        'byte_loop: for _ in 0..data_header.data_amount {
            for length in 1..=lookup_table.max_length {
                let bits = get_bits(input, offset, length);
                if let Some(value) = lookup_table.get(&(bits, length)) {
                    output.write_u8(*value).unwrap();
                    offset += length;
                    continue 'byte_loop;
                }
            }
            panic!("Encountered malformed data");
        }
        output.flush().unwrap();
    }
}
fn get_bits(input: &[u8], offset: usize, amount: usize) -> u8 {
    let byte_offset = offset >> 3;
    let bit_offset = offset & 0x7;
    match bit_offset + amount {
        0 => panic!("This should not be reachable"),
        1..=8 => input[byte_offset].wrapping_shr(bit_offset as u32) & ((1 << amount) - 1),
        9.. => {
            let bits_in_second_byte = bit_offset + amount - 8;
            let bits_in_first_byte = amount - bits_in_second_byte;
            let first_half = input[byte_offset].wrapping_shr(bit_offset as u32)
                & ((1 << bits_in_first_byte) - 1);
            (input[byte_offset + 1].wrapping_shl(bits_in_first_byte as u32) | first_half)
                & 1u8.wrapping_shl(amount as u32).wrapping_sub(1)
        }
    }
}
#[test]
fn test_decoding() {
    let data = "Hello, World!".as_bytes();
    let mut result = Vec::new();
    Huffman::encode(data, &mut BufWriter::new(&mut result));
    assert!(!result.is_empty());
    let data = &result;
    let mut result = Vec::new();
    Huffman::decode(data, &mut BufWriter::new(&mut result));
    println!("{result:?}");
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "Hello, World!");
}
