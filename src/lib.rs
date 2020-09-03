extern crate sha3;

mod vector;
use self::vector::{Vector, Dot};

pub struct Poly {}
pub struct ByteArray {}


////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
pub fn Kyber_CPAPKE_KeyGen() -> (ByteArray, ByteArray){
    unimplemented!();
}

// Encryption : public key, message, random coins => ciphertext
pub fn Kyber_CPAPKE_Enc(pk: ByteArray, m: ByteArray, r: ByteArray) -> ByteArray{
    unimplemented!();
}

// Decryption : secret key, ciphertext => message
pub fn Kyber_CPAPKE_Dec(sk: ByteArray, c: ByteArray) -> ByteArray{
    unimplemented!();
}

////////////// KEM /////////////////////////

// Kyber CCAKEM Key Generation => (secret key, public key)
pub fn Kyber_CCAKEM_KeyGen() -> (ByteArray, ByteArray){
    unimplemented!();
}

// Encryption : public key  => ciphertext, Shared Key
pub fn Kyber_CCAKEM_Enc(pk: ByteArray) -> (ByteArray, ByteArray) {
    unimplemented!();
}

// Decryption : secret key, ciphertext => Shared Key
pub fn Kyber_CCAKEM_Dec(c: ByteArray, sk: ByteArray) -> ByteArray{
    unimplemented!();
}

////////////////// Utils ////////////////////

// receives as input a byte stream B=(b0; b1; b2;...) and computes the NTT-representation a' = a'_0 + a'_0X + ... + a'_n-1X^(n-1) in R_q of a in R_q
pub fn parse(bs: ByteArray) -> Poly {
    unimplemented!();
}

// Centered Binomial Distribution
pub fn CBD(bs: ByteArray) -> Poly {
    unimplemented!();
}

// Serialize Polynomial into ByteArray
pub fn Encode(p: Poly) -> ByteArray {
    unimplemented!();
}

// Deserialize ByteArray into Polynomial
pub fn Decode(bs: ByteArray) -> Poly {
    unimplemented!();
}

// Pseudo random function => SHAKE-256(s||b);
fn PRF(){
    unimplemented!();
}

// Extendable output function => SHAKE-128
fn XOF(){
    unimplemented!();
}

// Hash function => SHA3-256
fn H(){
    unimplemented!();
}

// Hash function => SHA3-512
fn G(){
    unimplemented!();
}

// Key Derivation function => SHAKE-256
fn KDF(){
    unimplemented!();
}

// Number theoretic Transform
fn NTT(){
    unimplemented!();
}

// Reverse NTT
fn RevNTT(){
    unimplemented!();
}

fn Compress(x: u64, d: u64) -> u64 {
    unimplemented!();
}

fn Decompress(x: u64, d: u64) -> u64 {
    unimplemented!();
}