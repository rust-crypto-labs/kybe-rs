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
