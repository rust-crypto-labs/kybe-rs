extern crate sha3;

mod vector;
use self::vector::{Dot, Vector};

pub struct Poly {}
pub struct ByteArray {}

////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
pub fn kyber_cpapke_key_gen() -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Encryption : public key, message, random coins => ciphertext
pub fn kyber_cpapke_enc(pk: ByteArray, m: ByteArray, r: ByteArray) -> ByteArray {
    unimplemented!();
}

// Decryption : secret key, ciphertext => message
pub fn kyber_cpapke_dec(sk: ByteArray, c: ByteArray) -> ByteArray {
    unimplemented!();
}

////////////// KEM /////////////////////////

// Kyber CCAKEM Key Generation => (secret key, public key)
pub fn kyber_ccakem_key_gen() -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Encryption : public key  => ciphertext, Shared Key
pub fn kyber_ccakem_enc(pk: ByteArray) -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Decryption : secret key, ciphertext => Shared Key
pub fn kyber_ccakem_dec(c: ByteArray, sk: ByteArray) -> ByteArray {
    unimplemented!();
}

////////////////// Utils ////////////////////

// receives as input a byte stream B=(b0; b1; b2;...) and computes the NTT-representation a' = a'_0 + a'_0X + ... + a'_n-1X^(n-1) in R_q of a in R_q
pub fn parse(bs: ByteArray) -> Poly {
    unimplemented!();
}

// Centered Binomial Distribution
pub fn cbd(bs: ByteArray) -> Poly {
    unimplemented!();
}

// Serialize Polynomial into ByteArray
pub fn encode(p: Poly) -> ByteArray {
    unimplemented!();
}

// Deserialize ByteArray into Polynomial
pub fn decode(bs: ByteArray) -> Poly {
    unimplemented!();
}

// Pseudo random function => SHAKE-256(s||b);
fn prf() {
    unimplemented!();
}

// Extendable output function => SHAKE-128
fn xof() {
    unimplemented!();
}

// Hash function => SHA3-256
fn h() {
    unimplemented!();
}

// Hash function => SHA3-512
fn g() {
    unimplemented!();
}

// Key Derivation function => SHAKE-256
fn kdf() {
    unimplemented!();
}

// Number theoretic Transform
fn ntt() {
    unimplemented!();
}

// Reverse NTT
fn rev_ntt() {
    unimplemented!();
}

fn compress(x: u64, d: u64) -> u64 {
    unimplemented!();
}

fn decompress(x: u64, d: u64) -> u64 {
    unimplemented!();
}
