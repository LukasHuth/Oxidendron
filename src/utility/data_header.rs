use std::{
    io::{ErrorKind, Write},
    ops::RangeInclusive,
};

use crate::utility::{CURRENT_VERSION, Occurences};

pub(crate) struct DataHeader {
    pub(crate) version: u8,
    pub(crate) occurences: Occurences,
    pub(crate) data_amount: u64,
}
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum OccurrenceType {
    FullList {
        list: Occurences,
    },
    RangeList {
        ranges: Vec<RangeInclusive<u8>>,
        list: Vec<u64>,
        positional_list: Vec<(u8, u64)>,
    },
}
impl OccurrenceType {
    pub fn new(list: Occurences) -> Self {
        Self::FullList { list }
    }
    pub fn convert_to_full_list(&mut self) {
        match &*self {
            Self::FullList { .. } => (),
            Self::RangeList { .. } => self.convert_from_range_list_to_full_list(),
        }
    }
    pub fn convert_to_range_list(&mut self) {
        match &*self {
            Self::FullList { .. } => self.convert_from_full_list_to_range_list(),
            Self::RangeList { .. } => (),
        }
    }
    fn convert_from_range_list_to_full_list(&mut self) {
        let (ranges, list, positional_list) = match &*self {
            Self::RangeList {
                ranges,
                list,
                positional_list,
            } => (ranges.clone(), list, positional_list),
            Self::FullList { .. } => unreachable!(),
        };
        let mut list_offset = 0;
        let mut occurences = [0; 256];
        for range in ranges {
            for i in range {
                occurences[i as usize] = list[list_offset];
                list_offset += 1;
            }
        }
        for &(position, value) in positional_list {
            occurences[position as usize] = value;
        }
        *self = Self::FullList { list: occurences }
    }

    fn convert_from_full_list_to_range_list(&mut self) {
        let occurences = match &*self {
            Self::FullList { list } => list,
            Self::RangeList { .. } => unreachable!(),
        };
        let mut ranges: Vec<RangeInclusive<u8>> = Vec::new();
        let mut list: Vec<u64> = Vec::new();
        let mut positional_list: Vec<(u8, u64)> = Vec::new();
        let mut current_range_start: Option<usize> = None;
        for (i, &current_value) in occurences.iter().enumerate().take(256) {
            if current_value == 0 {
                if let Some(current_range_start) = current_range_start {
                    if current_range_start == i - 1 {
                        let value = list.pop().expect("Pop should always work, since a value has to be pushed for current_range_start to be set");
                        positional_list.push((i as u8 - 1, value));
                    } else {
                        ranges.push(current_range_start as u8..=i as u8 - 1);
                    }
                } else {
                    // last was empty and this is empty, so nothing has to be done
                    continue;
                }
                current_range_start = None;
            } else {
                list.push(occurences[i]);
                if current_range_start.is_none() {
                    current_range_start = Some(i);
                }
            }
        }
        *self = Self::RangeList {
            ranges,
            list,
            positional_list,
        }
    }
    pub const fn size(&self) -> usize {
        use std::mem::size_of;
        match self {
            Self::FullList { .. } => size_of::<u64>() * 256,
            Self::RangeList {
                ranges,
                list,
                positional_list,
            } => {
                size_of::<u8>()
                    + (size_of::<u8>() * 2 * ranges.len())
                    + (size_of::<u64>() * list.len())
                    + size_of::<u8>()
                    + ((size_of::<u8>() + size_of::<u64>()) * positional_list.len())
            }
        }
    }
}
#[test]
fn test_occurence_type() {
    let data = "Hello, World!".as_bytes();
    let occurrence = crate::utility::huffman_tree::HuffmanTree::find_occurences(data);
    let mut occurrence_type = OccurrenceType::new(occurrence);
    let occurence_clone = occurrence_type.clone();
    occurrence_type.convert_to_range_list();
    occurrence_type.convert_to_full_list();
    assert_eq!(occurrence_type, occurence_clone);
}
impl DataHeader {
    // pub(crate) const SIZE: usize =
    //     std::mem::size_of::<Occurences>() + std::mem::size_of::<u64>() + std::mem::size_of::<u8>();
    pub fn new(occurences: Occurences, data_amount: u64) -> Self {
        Self {
            version: CURRENT_VERSION,
            occurences,
            data_amount,
        }
    }
    pub fn write_to<W: Write + byteorder::WriteBytesExt>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
        use byteorder::BigEndian;
        writer.write_u8(CURRENT_VERSION)?;
        let mut occ_type = OccurrenceType::new(self.occurences);
        occ_type.convert_to_range_list();
        let (ranges, list, positional_list) = match occ_type {
            OccurrenceType::FullList { .. } => unreachable!(),
            OccurrenceType::RangeList {
                ranges,
                list,
                positional_list,
            } => (ranges, list, positional_list),
        };
        writer.write_u8(ranges.len() as u8)?;
        for range in ranges {
            writer.write_u8(*range.start())?;
            writer.write_u8(*range.end())?;
        }
        for item in list {
            writer.write_u64::<BigEndian>(item)?;
        }
        writer.write_u8(positional_list.len() as u8)?;
        for (position, amount) in positional_list {
            writer.write_u8(position)?;
            writer.write_u64::<BigEndian>(amount)?;
        }
        writer.write_u64::<BigEndian>(self.data_amount)
    }

    pub(crate) fn size(&self) -> usize {
        use std::mem::size_of;
        match self.version {
            0 => unreachable!("There is no version 0"),
            1 => size_of::<u8>() + size_of::<u64>() * 256 + size_of::<u64>(),
            2 => {
                let mut occ_type = OccurrenceType::new(self.occurences);
                occ_type.convert_to_range_list();
                size_of::<u8>() + size_of::<u64>() + occ_type.size()
            }
            3.. => {
                panic!("Encountered a newer version");
            }
        }
    }
    pub(crate) fn read_from(input: &[u8]) -> std::io::Result<Self> {
        let version = input[0];
        match version {
            0 => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Read an unexpected 0 as version",
            )),
            1 => Ok(read_v1_0::read_from(input)),
            2 => read_v1_1::read_from(input),
            3.. => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Read a newer file version than supported: {version}"),
            )),
        }
    }
}
mod read_v1_0;
mod read_v1_1;
