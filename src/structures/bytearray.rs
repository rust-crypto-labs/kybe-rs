//! ByteArray
//!
//! ByteArray used for exchange and encoding/decoding

use rand::prelude::*;
/// A struct representing an array of bytes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ByteArray {
    /// Array of bytes
    pub data: Vec<u8>,
}

impl ByteArray {
    /// Generate an empty ByteArray
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Generate a ByteArrey from a slice of bytes
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    /// Generate a ByteArray of size len filled with random values
    pub fn random(len: usize) -> Self {
        let mut data = vec![0; len];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut data);

        Self { data }
    }

    /// Append two ByteArrays together
    pub fn append(&self, other: &Self) -> Self {
        let mut data = Vec::with_capacity(self.data.len() + other.data.len());

        data.extend_from_slice(&self.data);
        data.extend_from_slice(&other.data);

        Self { data }
    }

    /// Append an array of ByteArrays together
    pub fn concat(items: &[&Self]) -> Self {
        let len = items.iter().map(|slice| slice.data.len()).sum();
        let mut data = Vec::with_capacity(len);

        for item in items.iter() {
            data.extend_from_slice(&item.data);
        }

        Self { data }
    }

    /// Get the value of the bit at position pos
    pub fn get_bit(&self, pos: usize) -> bool {
        let (index, offset) = (pos / 8, pos % 8);
        let mask = 1 << offset;
        (self.data[index] & mask) != 0
    }

    /// Trim the ByteArray from the first num bytes
    pub fn skip(&self, num: usize) -> Self {
        let data = if num < self.data.len() {
            Vec::from(&self.data[..num])
        } else {
            Vec::new()
        };

        Self { data }
    }

    /// Split the ByteArray at the position pos
    pub fn split_at(&self, pos: usize) -> (Self, Self) {
        let (d1, d2) = self.data.split_at(pos);
        (Self { data: d1.to_vec() }, Self { data: d2.to_vec() })
    }

    /// Truncate the Byte Array to size len
    pub fn truncate(&self, len: usize) -> Self {
        let mut data = self.data.clone();
        data.truncate(len);
        Self { data }
    }
}
