//! Encode/Decode functions
//!
//! Utils to serialize/deserialize polynomial and polyvec

use crate::{
    polyvec::structures::{FiniteField, RingModule},
    ByteArray, Poly3329, PolyVec3329, F3329,
};

/// Deserialize ByteArray into Polynomial
/// Algorithm 3 p. 8
pub fn decode_to_poly(bs: ByteArray) -> Poly3329 {
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

/// Serialize Poly into ByteArray
pub fn encode_poly(p: Poly3329) -> ByteArray {
    let b = ByteArray { data: vec![] };

    for i in 0..256 {
        let val = p[i].to_int().to_le_bytes();
        b.append(&ByteArray::from_bytes(&val));
    }
    b
}

/// Deserialize ByteArray into PolyVec
pub fn decode_to_polyvec(bs: ByteArray) -> PolyVec3329 {
    let ell = bs.data.len() / 256;
    let mut b = bs;
    let mut p_vec = PolyVec3329::from_vec(vec![Poly3329::init(256); ell]);

    for i in 0..ell {
        let (a, c) = b.split_at(ell);
        p_vec.set(i, decode_to_poly(a));
        b = c.clone();
    }

    p_vec
}

/// Serialize PolyVec into ByteArray
pub fn encode_polyvec(p_vec: PolyVec3329) -> ByteArray {
    let b = ByteArray { data: vec![] };
    let ell = p_vec.dimension();

    for i in 0..ell {
        let p = p_vec.get(i);
        b.append(&encode_poly(p));
    }

    b
}
