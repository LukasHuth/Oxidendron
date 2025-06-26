use std::io::Write;

use crate::utility::{Occurences, CURRENT_VERSION};

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

