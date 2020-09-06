use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};

pub fn shake_128(_d: Vec<u8>) -> Vec<u8> {
    unimplemented!();
}

pub fn shake_256(_d: Vec<u8>) -> Vec<u8> {
    unimplemented!();
}

pub fn sha3_256(d: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(d);
    hasher.finalize().to_vec()
}

pub fn sha3_512(d: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha3_512::new();
    hasher.update(d);
    hasher.finalize().to_vec()
}
