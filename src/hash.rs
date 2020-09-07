use crate::sha3::digest::ExtendableOutput;
use crate::sha3::digest::XofReader;
use crate::sha3::Digest;

use sha3::{Sha3XofReader, Sha3_256, Sha3_512, Shake128, Shake256};

pub fn shake_128(data: Vec<u8>, len: usize) -> Vec<u8> {
    unimplemented!();
    use crate::sha3::digest::Input;
    let mut buffer = vec![0; len];
    let mut shake: Shake128 = Default::default();
    shake.input(data);

    let mut reader = shake.xof_result();
    reader.read(&mut buffer);
    buffer
}

pub fn shake_256(data: Vec<u8>, len: usize) -> Vec<u8> {
    //unimplemented!();
    use crate::sha3::digest::Input;
    let mut buffer = vec![0; len];
    let mut shake: Shake256 = Default::default();
    shake.input(data);

    let mut reader = shake.xof_result();
    reader.read(&mut buffer);
    buffer
}

pub fn sha3_256(data: Vec<u8>) -> Vec<u8> {
    let mut hasher: Sha3_256 = Default::default();
    hasher.input(data);
    hasher.result().to_vec()
}

pub fn sha3_512(data: Vec<u8>) -> Vec<u8> {
    let mut hasher: Sha3_512 = Default::default();
    hasher.input(data);
    hasher.result().to_vec()
}
