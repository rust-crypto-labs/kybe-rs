use rand::prelude::*;
#[derive(Debug)]
pub struct ByteArray {
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

    pub fn get_bit(&self, pos: usize) -> bool {
        let byte_index = pos >> 3;
        let bit_in_byte = pos & 0b111;
        let mask = 2 << bit_in_byte;
        !(self.data[byte_index] & mask == 0)
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
