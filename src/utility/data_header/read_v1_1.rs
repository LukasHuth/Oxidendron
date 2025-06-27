use crate::utility::data_header::{DataHeader, OccurrenceType};

pub(super) fn read_from(input: &[u8]) -> std::io::Result<DataHeader> {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::mem::size_of;
    let version = input[0];

    assert_eq!(
        version, 2,
        "This code is only for version 2 of the file format"
    );

    let ranges_amount = input[1] as usize;
    let input = &input[2..];
    // read ranges
    let mut ranges = Vec::new();
    for i in 0..ranges_amount {
        let start = (&input[size_of::<u8>() * 2 * i..]).read_u8()?;
        let end = (&input[size_of::<u8>() * 2 * i + size_of::<u8>()..]).read_u8()?;
        ranges.push(start..=end);
    }
    // adjust input to continue from 0
    let input = &input[size_of::<u8>() * 2 * ranges_amount..];
    // list amount can be calculated by suming all range lengths (inclusive ranges)
    let list_amount = ranges
        .iter()
        .map(|range| *range.end() - *range.start() + 1)
        .sum::<u8>() as usize;
    // read list
    let mut list = Vec::new();
    for i in 0..list_amount {
        list.push((&input[size_of::<u64>() * i..]).read_u64::<BigEndian>()?);
    }
    // adjust input to continue from 0
    let input = &input[size_of::<u64>() * list_amount..];
    // read positional data (list of single position, value pairs)
    let positional_list_len = input[0] as usize;
    let mut positional_list = Vec::new();
    let input = &input[1..];
    for i in 0..positional_list_len {
        let offset = (size_of::<u8>() + size_of::<u64>()) * i;
        let position = input[offset];
        let value = (&input[offset + 1..]).read_u64::<BigEndian>()?;
        positional_list.push((position, value));
    }
    // adjust input to continue from 0
    let input = &input[(size_of::<u8>() + size_of::<u64>()) * positional_list_len..];
    let data_amount = (&input[..]).read_u64::<BigEndian>()?;
    let mut occ_type = OccurrenceType::RangeList {
        ranges,
        list,
        positional_list,
    };
    occ_type.convert_to_full_list();
    let occurences = match occ_type {
        OccurrenceType::FullList { list } => list,
        OccurrenceType::RangeList { .. } => unreachable!(),
    };
    Ok(DataHeader {
        version,
        occurences,
        data_amount,
    })
}
