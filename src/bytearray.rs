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
        let mut data = self.data.clone();
        data.extend(other.data.iter().cloned());
        Self { data }
    }

    pub fn concat(items: &[&Self]) -> Self {
        if items.len() == 0 {
            return Self { data: vec![] };
        }

        let mut data = items.first().unwrap().data.clone();
        for item in items.iter().skip(1) {
            data.extend(item.data.iter().cloned());
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
}
