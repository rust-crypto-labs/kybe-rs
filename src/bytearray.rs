use rand::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub struct ByteArray {
    /// Array of bytes
    pub data: Vec<u8>,
}

impl ByteArray {
    pub fn random(len: usize) -> Self {
        let mut data = vec![0; len];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut data);

        Self { data }
    }

    pub fn append(&self, other: &Self) -> Self {
        let mut data = Vec::with_capacity(self.data.len() + other.data.len());

        data.extend_from_slice(&self.data);
        data.extend_from_slice(&other.data);

        Self { data }
    }

    pub fn concat(items: &[&Self]) -> Self {
        let len = items.iter().map(|slice| slice.data.len()).sum();
        let mut data = Vec::with_capacity(len);

        for item in items.iter() {
            data.extend_from_slice(&item.data);
        }

        Self { data }
    }

    pub fn get_bit(&self, pos: usize) -> bool {
        let byte_index = pos >> 3;
        let bit_in_byte = pos & 0b111;
        let mask = 2 << bit_in_byte;
        !(self.data[byte_index] & mask == 0)
    }

    /// Builds a byte array from an integer
    /// Note: little endian
    pub fn from_int(x: u64) -> Self {
        let mut data = vec![0; 8];
        for i in 0..8 {
            data.push(((x >> (8 * i)) & 0xf) as u8);
        }

        Self { data }
    }

    /// Exports the byte array as an integer
    /// Note: little endian
    pub fn to_int(&self) -> u64 {
        assert!(self.data.len() <= 8);
        let mut result = 0;
        for (i, x) in self.data.iter().enumerate() {
            result += (*x as u64) << (8 * i);
        }
        result
    }
}
