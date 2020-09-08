extern crate sha3;

mod bytearray;
mod hash;
mod polyvec;
mod primefield;

use hash::{sha3_256, sha3_512, shake_128, shake_256};

use polyvec::structures::RingModule;

use polyvec::{Matrix, PolyVec, Polynomial};
use primefield::PrimeField3329;

pub use bytearray::ByteArray;

pub type F3329 = PrimeField3329;
pub type Poly3329 = Polynomial<F3329>;
pub type PolyVec3329 = PolyVec<Poly3329>;
pub type PolyMatrix3329 = Matrix<Poly3329, PolyVec3329>;
type KyberParams = (usize, usize, usize);

////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
pub fn kyber_cpapke_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let (k, _, _) = params;
    let d = ByteArray::random();
    let (rho, sigma) = g(d);

    let mut a = PolyMatrix3329::init_matrix(k, k);

    for i in 0..k {
        for j in 0..k {
            a.set(j, i, parse(xof(&rho, j, i)));
        }
    }

    let (mut s, mut e) = (PolyVec3329::init(256), PolyVec3329::init(256));
    for i in 0..k {
        s.set(i, cbd(prf(&sigma, i)));
        e.set(i, cbd(prf(&sigma, k + i)));
    }
    let s_hat = ntt(s);
    let e_hat = ntt(e);

    let t_hat = a.vec_mul(&s_hat).add(&e_hat);

    // mod+ q  ?
    let sk = encode(t_hat).append(&rho);
    let pk = encode(s_hat);

    (sk, pk)
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
fn parse(_bs: ByteArray) -> Poly3329 {
    unimplemented!();
}

// Centered Binomial Distribution
fn cbd(_bs: ByteArray) -> Poly3329 {
    unimplemented!();
}

// Serialize Polynomial into ByteArray
fn encode(_p: PolyVec3329) -> ByteArray {
    unimplemented!();
}

// Deserialize ByteArray into Polynomial
fn decode(_bs: ByteArray) -> PolyVec3329 {
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
fn ntt(_p: PolyVec3329) -> PolyVec3329 {
    unimplemented!();
}

// Reverse NTT
fn rev_ntt(_p_hat: PolyVec3329) -> PolyVec3329 {
    unimplemented!();
}

fn compress(_x: usize, _d: usize) -> usize {
    unimplemented!();
}

fn decompress(_x: usize, _d: usize) -> usize {
    unimplemented!();
}
