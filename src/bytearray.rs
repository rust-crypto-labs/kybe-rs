use rand::prelude::*;
#[derive(Debug)]
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

impl PartialEq for ByteArray {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for (i, c) in self.data.iter().enumerate() {
            if other.data[i] != *c {
                return false;
            }
        }
        true
    }
}
impl Eq for ByteArray {}
