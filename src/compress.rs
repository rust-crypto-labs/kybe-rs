use crate::{PrimeField3329, Poly3329, PolyVec3329};

/// Compress function on coefficients, p. 6
pub fn compress_integer(x: i64, d: usize, q: usize) -> i64 {
    let m = 1 << d;
    let f = (m as f64)/(q as f64);
    let f = f * (x as f64);

    let mut c = (f.round() as i64) % m;

    // We now need to compute the mod+ variant; this is not
    // constant time
    if 2*c >= m {
        c -= m;
    }

    c
}

/// Decompress function on coefficients, p. 6
pub fn decompress_integer(x: i64, d: usize, q: usize) -> i64 {
    let m = 1 << d;
    let f = (q as f64 * x as f64)/(m as f64);
    
    f.round() as i64
}

/// Compress function on R_q
pub fn compress_poly(x: Poly3329, d: usize, q: usize) -> Poly3329 {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(PrimeField3329::from_int(compress_integer(xi.to_int(), d, q)));
    }
    Poly3329::from_vec(coeffs, x.dimension())
}

/// Deompress function on R_q
pub fn decompress_poly(x: Poly3329, d: usize, q: usize) -> Poly3329 {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(PrimeField3329::from_int(decompress_integer(xi.to_int(), d, q)));
    }
    Poly3329::from_vec(coeffs, x.dimension())
}

/// Compress function on R_q^k
pub fn compress_polyvec(x: PolyVec3329, d: usize, q: usize) -> PolyVec3329 {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(compress_poly(xi.clone(), d, q));
    }
    PolyVec3329::from_vec(coeffs)
}


/// Decompress function on R_q^k
fn decompress_polyvec(x: PolyVec3329, d: usize, q: usize) -> PolyVec3329 {
    let mut coeffs = vec![];
    for xi in x.coefficients.iter() {
        coeffs.push(decompress_poly(xi.clone(), d, q));
    }
    PolyVec3329::from_vec(coeffs)
}