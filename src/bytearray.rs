use rand::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ByteArray {
    /// Array of bytes
    pub data: Vec<u8>,
}

impl ByteArray {
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

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

    pub fn skip(&self, num: usize) -> Self {
        Self {
            data: self.data.iter().cloned().skip(num).collect(),
        }
    }

    pub fn split_at(&self, pos: usize) -> (Self, Self) {
        let (d1, d2) = self.data.split_at(pos);
        (Self { data: d1.to_vec() }, Self { data: d2.to_vec() })
    }

    pub fn truncate(&self, len: usize) -> Self {
        let mut data = self.data.clone();
        data.truncate(len);
        Self { data }
    }
}
