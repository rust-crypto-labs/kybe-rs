extern crate sha3;

use std::convert::TryInto;

mod bytearray;
mod hash;
mod polyvec;
mod primefield;

use polyvec::structures::{FiniteField, RingModule};

use polyvec::{Matrix, PolyVec, Polynomial};
use primefield::PrimeField3329;

pub use bytearray::ByteArray;

pub type F3329 = PrimeField3329;
pub type Poly3329 = Polynomial<F3329>;
pub type PolyVec3329 = PolyVec<Poly3329>;
pub type PolyMatrix3329 = Matrix<Poly3329>;

pub struct KyberParams {
    pub n: usize,
    pub k: usize,
    pub q: usize,
    pub eta: usize,
    pub du: usize,
    pub dv: usize,
    pub delta: usize,
}

impl KyberParams {
    pub const fn kyber512() -> Self {
        Self {
            n: 256,
            k: 2,
            q: 3329,
            eta: 2,
            du: 10,
            dv: 3,
            delta: 178,
        }
    }
}

////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
// Algorithm 4 p. 9
pub fn kyber_cpapke_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let k = params.k;
    let d = ByteArray::random(32);
    let (rho, sigma) = g(d);

    let mut a = PolyMatrix3329::init_matrix(k, k);

    let XOF_LEN = 4000;

    for i in 0..k {
        for j in 0..k {
            a.set(j, i, parse(xof(&rho, j, i, XOF_LEN), params.n, params.q));
        }
    }

    let (mut s, mut e) = (PolyVec3329::init(256), PolyVec3329::init(256));
    let prf_len = 64 * params.eta;

    for i in 0..k {
        s.set(i, cbd(prf(&sigma, i, prf_len), params.eta));
        e.set(i, cbd(prf(&sigma, k + i, prf_len), params.eta));
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
// Algorithm 7 p. 11
pub fn kyber_ccakem_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let z = ByteArray::random(32);
    let (pk, sk) = kyber_cpapke_key_gen(params);
    let (h1, h2) = h(&pk);
    let sk = sk.append(&pk).append(&h1).append(&h2).append(&z);
    (sk, pk)
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
// Algorithm 1 p. 7
fn parse(bs: ByteArray, n: usize, q: usize) -> Poly3329 {
    let mut i = 0;
    let mut j = 0;
    let mut coeffs = vec![F3329::default(); n];
    while j < n {
        let d = (bs.data[i] as usize) + (bs.data[i + 1] as usize) << 8;
        if d < 19 * q {
            coeffs[j] = F3329::from_int(d.try_into().unwrap());
            j += 1;
        }
        i += 2;
    }
    Poly3329::from_vec(coeffs, n)
}

// Centered Binomial Distribution
// Algorithm 2 p. 8
// Takes as input an array of 64 eta bytes
fn cbd(bs: ByteArray, eta: usize) -> Poly3329 {
    let mut f_coeffs = vec![F3329::default(); 256];
    for i in 0..256 {
        let mut a = 0;
        let mut b = 0;

        for j in 0..eta {
            if bs.get_bit(2 * i * eta + j) {
                a += 1;
            }
            if bs.get_bit(2 * i * eta + eta + j) {
                b += 1;
            }
        }

        f_coeffs[i] = F3329::from_int(a - b);
    }
    Poly3329::from_vec(f_coeffs, 256)
}

// Deserialize ByteArray into Polynomial
// Algorithm 3 p. 8
fn decode(bs: ByteArray) -> Poly3329 {
    let ell = bs.data.len() / 32;
    let f = vec![F3329::from_int(0); 256];

    for i in 0..256 {
        for j in 0..ell {
            if bs.get_bit(i * ell + j) {
                f[i].add(&F3329::from_int(2 << j));
            }
        }
    }
    Poly3329::from_vec(f, 256)
}

// Serialize Polynomial into ByteArray
fn encode(_p: PolyVec3329) -> ByteArray {
    unimplemented!();
}

// Pseudo random function => SHAKE-256(s||b);
fn prf(s: &ByteArray, b: usize, len: usize) -> ByteArray {
    let b_as_bytes = ByteArray {
        data: (b as u64).to_be_bytes().to_vec(),
    };
    let input = s.clone().append(&b_as_bytes);
    ByteArray {
        data: hash::shake_256(input.data, len),
    }
}

// Extendable output function => SHAKE-128(rho||j||i) with output of lenght len
fn xof(r: &ByteArray, j: usize, i: usize, len: usize) -> ByteArray {
    let i_as_bytes = ByteArray {
        data: (i as u64).to_be_bytes().to_vec(),
    };
    let j_as_bytes = ByteArray {
        data: (j as u64).to_be_bytes().to_vec(),
    };

    let input = r.clone().append(&j_as_bytes).append(&i_as_bytes);
    ByteArray {
        data: hash::shake_128(input.data, len),
    }
}

// Hash function => SHA3-256
fn h(r: &ByteArray) -> (ByteArray, ByteArray) {
    let hash = hash::sha3_256(r.data.clone());
    let (part0, part1) = hash.split_at(16);

    (
        ByteArray {
            data: part0.to_vec(),
        },
        ByteArray {
            data: part1.to_vec(),
        },
    )
}

// Hash function => SHA3-512
fn g(r: ByteArray) -> (ByteArray, ByteArray) {
    let hash = hash::sha3_512(r.data);
    let (part0, part1) = hash.split_at(32);

    (
        ByteArray {
            data: part0.to_vec(),
        },
        ByteArray {
            data: part1.to_vec(),
        },
    )
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
