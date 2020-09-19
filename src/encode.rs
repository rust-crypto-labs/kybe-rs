use crate::{polyvec::structures::FiniteField, ByteArray, Poly3329, PolyVec3329, F3329};

// Deserialize ByteArray into Polynomial
// Algorithm 3 p. 8
pub fn decode_to_poly(bs: &ByteArray) -> Poly3329 {
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

// Serialize Poly into ByteArray
pub fn encode_poly(_p: Poly3329) -> ByteArray {
    unimplemented!()
}

// Deserialize ByteArray into PolyVec
pub fn decode_to_polyvec(_bs: &ByteArray) -> PolyVec3329 {
    unimplemented!()
}

// Serialize PolyVec into ByteArray
pub fn encode_polyvec(_p: PolyVec3329) -> ByteArray {
    unimplemented!()
}
