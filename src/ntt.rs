use crate::polyvec::structures::{RingModule, FiniteRing};
use crate::{Poly3329, PolyMatrix3329, PolyVec3329};

/// Basecase multiplication between polynomials (p 7)
pub fn bcm(_a: &Poly3329, _b: &Poly3329) -> Poly3329 {
    unimplemented!()
}

/// Matrix basecase multiplication, cf p. 7
pub fn bcm_matrix_vec(_a: &PolyMatrix3329, _b: &PolyVec3329) -> PolyVec3329 {
    unimplemented!()
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product(a_hat: &Poly3329, b_hat: &Poly3329) -> Poly3329 {
    rev_ntt(&bcm(a_hat, b_hat))
}

/// Computes a^T.b as NTT^-1(a_hat^T o b_hat)
pub fn ntt_product_vec(a_hat: &PolyVec3329, b_hat: &PolyVec3329) -> Poly3329 {

    let l = a_hat.dimension();
    assert_eq!(l, b_hat.dimension());

    let mut p = bcm(&a_hat.get(0), &b_hat.get(0));
    for i in 1..l {
        p = p.add(&bcm(&a_hat.get(i), &b_hat.get(i)));
    }
    rev_ntt(&p)
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product_matvec(_a_hat: &PolyMatrix3329, _b_hat: &PolyVec3329) -> PolyVec3329 {
    unimplemented!()
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
pub fn ntt(_p: &Poly3329) -> Poly3329 {
    unimplemented!()
}

// Reverse NTT
pub fn rev_ntt(_p_hat: &Poly3329) -> Poly3329 {
    unimplemented!();
}
