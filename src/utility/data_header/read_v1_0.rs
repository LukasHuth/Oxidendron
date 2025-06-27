use crate::utility::{Occurences, data_header::DataHeader};
use std::mem::size_of;

const SIZE: usize = size_of::<Occurences>() + size_of::<u64>() + size_of::<u8>();

pub(crate) fn read_from(input: &[u8]) -> DataHeader {
    let version = input[0];
    assert_eq!(version, 1, "This code is only for version 1 of the file format");
    let occurences: Occurences = input[1..]
        .chunks_exact(std::mem::size_of::<u64>())
        .take(256)
        .map(|arr| u64::from_be_bytes(unsafe { arr.try_into().unwrap_unchecked() }))
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    let data_amount = u64::from_be_bytes(input[SIZE - 8..SIZE].to_vec().try_into().unwrap());
    DataHeader {
        version,
        occurences,
        data_amount,
    }
}
