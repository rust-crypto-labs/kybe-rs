//! Compress/Decompress functions
//!
//! Utils for compressing/decompressing integers, polynomials and polyvec

use crate::{Poly3329, PolyVec3329, F3329};

/// Compress function on coefficients, p. 6
pub fn compress_integer(x: usize, d: usize, q: usize) -> usize {
    let m = 1 << d;
    let f = (m as f64) / (q as f64);
    let f = f * (x as f64);

    (f.round() as usize) % m
}

/// Decompress function on coefficients, p. 6
pub fn decompress_integer(x: usize, d: usize, q: usize) -> usize {
    let m = 1 << d;
    let f = (q as f64 * x as f64) / (m as f64);

    f.round() as usize
}

/// Compress function on R_q
pub fn compress_poly<const N: usize>(x: Poly3329<N>, d: usize, q: usize) -> Poly3329<N> {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(F3329::from_int(compress_integer(xi.to_int(), d, q)));
    }
    Poly3329::from_vec(coeffs)
}

/// Deompress function on R_q
pub fn decompress_poly<const N: usize>(x: Poly3329<N>, d: usize, q: usize) -> Poly3329<N> {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(F3329::from_int(decompress_integer(xi.to_int(), d, q)));
    }
    Poly3329::from_vec(coeffs)
}

/// Compress function on R_q^k
pub fn compress_polyvec<const N: usize, const D: usize>(
    x: PolyVec3329<N, D>,
    d: usize,
    q: usize,
) -> PolyVec3329<N, D> {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(compress_poly(xi.clone(), d, q));
    }
    PolyVec3329::from_vec(coeffs)
}

/// Decompress function on R_q^k
pub fn decompress_polyvec<const N: usize, const D: usize>(
    x: PolyVec3329<N, D>,
    d: usize,
    q: usize,
) -> PolyVec3329<N, D> {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(decompress_poly(xi.clone(), d, q));
    }
    PolyVec3329::from_vec(coeffs)
}
