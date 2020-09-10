extern crate sha3;

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
type KyberParams = (usize, usize, usize);

////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
pub fn kyber_cpapke_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let D_SIZE = 4;

    let (k, _, _) = params;
    let d = ByteArray::random(D_SIZE);
    let (rho, sigma) = g(d);

    let mut a = PolyMatrix3329::init_matrix(k, k);

    let XOF_LEN = 4;
    let PARSE_N = 4;
    let PARSE_Q = 4;

    for i in 0..k {
        for j in 0..k {
            a.set(j, i, parse(xof(&rho, j, i, XOF_LEN), PARSE_N, PARSE_Q));
        }
    }

    let (mut s, mut e) = (PolyVec3329::init(256), PolyVec3329::init(256));
    let PRF_LEN = 4;
    let CBD_ETA = 4;
    let CBD_Q = 4;

    for i in 0..k {
        s.set(i, cbd(prf(&sigma, i, PRF_LEN), CBD_ETA, CBD_Q));
        e.set(i, cbd(prf(&sigma, k + i, PRF_LEN), CBD_ETA, CBD_Q));
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
// Algorithm 1 p. 7
fn parse(bs: ByteArray, degree: usize, q: usize) -> Poly3329 {
    let mut i = 0;
    let mut j = 0;
    let mut coeffs = vec![F3329::default(); degree];
    while j < degree {
        let d = (bs.data[i] as usize) + (bs.data[i + 1] as usize) << 8;
        if d < 19 * q {
            coeffs[j] = F3329::from_int(d);
            j += 1;
        }
        i += 2;
    }
    Poly3329::from_vec(coeffs, degree)
}

// Centered Binomial Distribution
fn cbd(_bs: ByteArray, _eta: usize, _q: usize) -> Poly3329 {
    unimplemented!();
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

// From https://doc.rust-lang.org/nomicon/borrow-splitting.html
pub fn split_at_mut<T>(v: &mut Vec<T>, mid: usize) -> (&mut [T], &mut [T]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();

    unsafe {
        assert!(mid <= len);

        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Hash function => SHA3-256
fn h(r: ByteArray) -> (ByteArray, ByteArray) {
    let mut hash = hash::sha3_256(r.data);
    let (part0, part1) = split_at_mut(&mut hash, 16);
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
    let mut hash = hash::sha3_512(r.data);
    let (part0, part1) = split_at_mut(&mut hash, 32);
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
