use crate::{Poly3329, PolyMatrix3329, PolyVec3329};

/// Basecase multiplication between polynomials (p 7)
pub fn bcm(a: &Poly3329, b: &Poly3329) -> Poly3329 {
    unimplemented!()
}

/// Matrix basecase multiplication, cf p. 7
pub fn bcm_matrix_vec(a: &PolyMatrix3329, b: &PolyVec3329) -> PolyVec3329 {
    unimplemented!()
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product(a_hat: &Poly3329, b_hat: &Poly3329) -> Poly3329 {
    rev_ntt(&bcm(a_hat, b_hat))
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product_vec(a_hat: &PolyVec3329, b_hat: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for (a_i, b_i) in a_hat.coefficients.iter().zip(b_hat.coefficients.iter()) {
        c.push(ntt_product(a_i, b_i));
    }
    PolyVec3329::from_vec(c)
}

// Number theoretic Transform on vectors
pub fn ntt_vec(p: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for p_i in p.coefficients.iter() {
        c.push(ntt(p_i));
    }
    PolyVec3329::from_vec(c)
}

// Reverse NTT on vectors
pub fn rev_ntt_vec(p_hat: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for p_i in p_hat.coefficients.iter() {
        c.push(rev_ntt(p_i));
    }
    PolyVec3329::from_vec(c)
}

// Number theoretic Transform
pub fn ntt(p: &Poly3329) -> Poly3329 {
    unimplemented!();
}

// Reverse NTT
pub fn rev_ntt(p_hat: &Poly3329) -> Poly3329 {
    unimplemented!();
}
