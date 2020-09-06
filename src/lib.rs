extern crate sha3;

mod polyvec;

use polyvec::{vector::Vector, Polymatrix3329, Polynom3329, Polyvec3329};
#[derive(Debug)]
pub struct ByteArray {}

impl ByteArray {
    pub fn random() -> Self {
        unimplemented!()
    }
}

impl PartialEq for ByteArray {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}
impl Eq for ByteArray {}

////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
pub fn kyber_cpapke_key_gen() -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Encryption : public key, message, random coins => ciphertext
pub fn kyber_cpapke_enc(_pk: &ByteArray, _m: &ByteArray, _r: ByteArray) -> ByteArray {
    unimplemented!();
}

// Decryption : secret key, ciphertext => message
pub fn kyber_cpapke_dec(_sk: &ByteArray, _c: &ByteArray) -> ByteArray {
    unimplemented!();
}

////////////// KEM /////////////////////////

// Kyber CCAKEM Key Generation => (secret key, public key)
pub fn kyber_ccakem_key_gen() -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Encryption : public key  => ciphertext, Shared Key
pub fn kyber_ccakem_enc(_pk: &ByteArray) -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Decryption : secret key, ciphertext => Shared Key
pub fn kyber_ccakem_dec(_c: &ByteArray, _sk: &ByteArray) -> ByteArray {
    unimplemented!();
}

////////////////// Utils ////////////////////

// receives as input a byte stream B=(b0; b1; b2;...) and computes the NTT-representation a' = a'_0 + a'_0X + ... + a'_n-1X^(n-1) in R_q of a in R_q
fn parse(_bs: ByteArray) -> Polynom3329 {
    unimplemented!();
}

// Centered Binomial Distribution
fn cbd(_bs: ByteArray) -> Polynom3329 {
    unimplemented!();
}

// Serialize Polynomial into ByteArray
fn encode(_p: Polyvec3329) -> ByteArray {
    unimplemented!();
}

// Deserialize ByteArray into Polynomial
fn decode(_bs: ByteArray) -> Polyvec3329 {
    unimplemented!();
}

// Pseudo random function => SHAKE-256(s||b);
fn prf(_s: &ByteArray, _b: usize) -> ByteArray {
    unimplemented!();
}

// Extendable output function => SHAKE-128
fn xof(_r: &ByteArray, _j: usize, _i: usize) -> ByteArray {
    unimplemented!();
}

// Hash function => SHA3-256
fn h(_r: ByteArray) -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Hash function => SHA3-512
fn g(_r: ByteArray) -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Key Derivation function => SHAKE-256
fn kdf() {
    unimplemented!();
}

// Number theoretic Transform
fn ntt(_p: Polyvec3329) -> Polyvec3329 {
    unimplemented!();
}

// Reverse NTT
fn rev_ntt(_p_hat: Polyvec3329) -> Polyvec3329 {
    unimplemented!();
}

fn compress(_x: usize, _d: usize) -> usize {
    unimplemented!();
}

fn decompress(_x: usize, _d: usize) -> usize {
    unimplemented!();
}
