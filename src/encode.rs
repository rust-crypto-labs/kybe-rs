//! Encode/Decode functions
//!
//! Utils to serialize/deserialize polynomial and polyvec

use crate::{
    polyvec::structures::{FiniteField, RingModule},
    ByteArray, Poly3329, PolyVec3329, F3329,
};

/// Deserialize ByteArray into Polynomial
/// Algorithm 3 p. 8
pub fn decode_to_poly(bs: ByteArray, ell: usize) -> Poly3329 {
    let mut f = vec![F3329::from_int(0); 256];

    for i in 0..256 {
        for j in 0..ell {
            if bs.get_bit(i * ell + j) {
                f[i] = f[i].add(&F3329::from_int(2 << j));
            }
        }
    }
    Poly3329::from_vec(f, 256)
}

/// Serialize Poly into ByteArray
pub fn encode_poly(p: Poly3329, ell: usize) -> ByteArray {
    let mut b = ByteArray::new();
    let (q,r) = (ell / 8, ell % 8);
    let m = (2 << r) - 1;

    for i in 0..256 {
        let mut v = p[i].to_int().to_le_bytes();
        v[q] &= m;
        b = b.append(&ByteArray::from_bytes(v.get(0..=q).unwrap()));
    }
    b
}

/// Deserialize ByteArray into PolyVec
pub fn decode_to_polyvec(bs: ByteArray, ell: usize) -> PolyVec3329 {
    let k = bs.data.len() / (32 * ell);
    let mut b = bs;
    let mut p_vec = PolyVec3329::from_vec(vec![Poly3329::init(256); k]);

    for i in 0..k {
        let (a, c) = b.split_at(32 * ell);
        p_vec.set(i, decode_to_poly(a, ell));
        b = c.clone();
    }

    p_vec
}

/// Serialize PolyVec into ByteArray
pub fn encode_polyvec(p_vec: PolyVec3329, s: usize) -> ByteArray {
    let mut b = ByteArray::new();
    let ell = p_vec.dimension();

    for i in 0..ell {
        let p = p_vec.get(i);
        b = b.append(&encode_poly(p, s));
    }

    b
}
