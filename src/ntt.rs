//! Number Theoretic Trasform (NTT)
//!
//! NTT operations and operations performed in the NTT domain

use crate::polyvec::structures::{FiniteField, FiniteRing, RingModule};
use crate::{Poly3329, PolyMatrix3329, PolyVec3329, F3329};
use std::convert::TryInto;

/// 128-roots of unity
const ZETAS_128: [i64; 128] = [
    2285, 2571, 2970, 1812, 1493, 1422, 287, 202, 3158, 622, 1577, 182, 962, 2127, 1855, 1468, 573,
    2004, 264, 383, 2500, 1458, 1727, 3199, 2648, 1017, 732, 608, 1787, 411, 3124, 1758, 1223, 652,
    2777, 1015, 2036, 1491, 3047, 1785, 516, 3321, 3009, 2663, 1711, 2167, 126, 1469, 2476, 3239,
    3058, 830, 107, 1908, 3082, 2378, 2931, 961, 1821, 2604, 448, 2264, 677, 2054, 2226, 430, 555,
    843, 2078, 871, 1550, 105, 422, 587, 177, 3094, 3038, 2869, 1574, 1653, 3083, 778, 1159, 3182,
    2552, 1483, 2727, 1119, 1739, 644, 2457, 349, 418, 329, 3173, 3254, 817, 1097, 603, 610, 1322,
    2044, 1864, 384, 2114, 3193, 1218, 1994, 2455, 220, 2142, 1670, 2144, 1799, 2051, 794, 1819,
    2475, 2459, 478, 3221, 3021, 996, 991, 958, 1869, 1522, 1628,
];

/// 128-roots of unity inversed
const ZETAS_INV_128: [i64; 128] = [
    1701, 1807, 1460, 2371, 2338, 2333, 308, 108, 2851, 870, 854, 1510, 2535, 1278, 1530, 1185,
    1659, 1187, 3109, 874, 1335, 2111, 136, 1215, 2945, 1465, 1285, 2007, 2719, 2726, 2232, 2512,
    75, 156, 3000, 2911, 2980, 872, 2685, 1590, 2210, 602, 1846, 777, 147, 2170, 2551, 246, 1676,
    1755, 460, 291, 235, 3152, 2742, 2907, 3224, 1779, 2458, 1251, 2486, 2774, 2899, 1103, 1275,
    2652, 1065, 2881, 725, 1508, 2368, 398, 951, 247, 1421, 3222, 2499, 271, 90, 853, 1860, 3203,
    1162, 1618, 666, 320, 8, 2813, 1544, 282, 1838, 1293, 2314, 552, 2677, 2106, 1571, 205, 2918,
    1542, 2721, 2597, 2312, 681, 130, 1602, 1871, 829, 2946, 3065, 1325, 2756, 1861, 1474, 1202,
    2367, 3147, 1752, 2707, 171, 3127, 3042, 1907, 1836, 1517, 359, 758, 1441,
];

/// 7-byte reversal (to impleme)
pub fn byte_rev(i: usize) -> usize {
    i
}

/// Basecase multiplication between polynomials (p 7)
pub fn bcm(a: &Poly3329, b: &Poly3329) -> Poly3329 {
    assert_eq!(a.degree(), b.degree());
    assert_eq!(a.dimension(), b.dimension());

    let d: usize = a.degree().try_into().unwrap();

    let mut p = Poly3329::init(a.dimension());

    // We assume d is even since spec requires operating mod X^2-zeta
    for i in (0..d).step_by(2) {
        let zeta = F3329::from_int(ZETAS_128[(2 * byte_rev(i / 2) + 1) % 128]);

        let p01 = a[i].mul(&b[i]);
        let p02 = a[i + 1].mul(&b[i + 1]).mul(&zeta);
        p[i] = p01.add(&p02);

        let p11 = a[i].mul(&b[i + 1]);
        let p12 = a[i + 1].mul(&b[i]);
        p[i + 1] = p11.add(&p12);
    }
    p
}

/// Base case multiplivation for vectors
pub fn bcm_vec(a: &PolyVec3329, b: &PolyVec3329) -> Poly3329 {
    let l = a.dimension();
    assert_eq!(l, b.dimension());

    let mut p = bcm(&a.get(0), &b.get(0));
    for i in 1..l {
        p = p.add(&bcm(&a.get(i), &b.get(i)));
    }
    p
}

/// Matrix basecase multiplication, cf p. 7
pub fn bcm_matrix_vec(a: &PolyMatrix3329, b: &PolyVec3329) -> PolyVec3329 {
    let (x, y) = a.dimensions();
    assert_eq!(x, b.dimension());

    let mut v = PolyVec3329::init(y);

    for i in 0..y {
        v.set(i, bcm_vec(&a.row(i), &b))
    }

    v
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product(a_hat: &Poly3329, b_hat: &Poly3329) -> Poly3329 {
    rev_ntt(&bcm(a_hat, b_hat))
}

/// Computes a^T.b as NTT^-1(a_hat^T o b_hat)
pub fn ntt_product_vec(a_hat: &PolyVec3329, b_hat: &PolyVec3329) -> Poly3329 {
    rev_ntt(&bcm_vec(a_hat, b_hat))
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product_matvec(a_hat: &PolyMatrix3329, b_hat: &PolyVec3329) -> PolyVec3329 {
    rev_ntt_vec(&bcm_matrix_vec(a_hat, b_hat))
}

/// Number theoretic Transform on vectors
pub fn ntt_vec(p: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for p_i in p.coefficients.iter() {
        c.push(ntt(p_i));
    }
    PolyVec3329::from_vec(c)
}

/// Reverse NTT on vectors
pub fn rev_ntt_vec(p_hat: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for p_i in p_hat.coefficients.iter() {
        c.push(rev_ntt(p_i));
    }
    PolyVec3329::from_vec(c)
}

/// Number theoretic Transform
pub fn ntt(p: &Poly3329) -> Poly3329 {
    let mut a = Poly3329::init(p.dimension());
    let d: usize = p.degree().try_into().unwrap();

    // We assume d is even since spec requires operating mod X^2-zeta
    for i in (0..d).step_by(2) {
        let mut p0 = p[0];
        let mut p1 = p[1];

        for j in (2..d).step_by(2) {
            let zeta = F3329::from_int(ZETAS_128[((2 * byte_rev(i / 2) + 1) * j) % 128]);
            let c0 = p[j].mul(&zeta);
            let c1 = p[j + 1].mul(&zeta);

            p0 = p0.add(&c0);
            p1 = p1.add(&c1);
        }
        a[i] = p0;
        a[i + 1] = p1;
    }

    a
}

/// Reverse NTT
pub fn rev_ntt(p_hat: &Poly3329) -> Poly3329 {
    let mut a = Poly3329::init(p_hat.dimension());

    if p_hat.degree() < 0 {
        // Zero polynomial's NTT is zero
        return p_hat.clone()
    }

    let d: usize = p_hat.degree().try_into().unwrap();
    let coeff = F3329::from_int((p_hat.degree() / 2).into());

    // We assume d is even since spec requires operating mod X^2-zeta
    for i in (2..d).step_by(2) {
        let mut p0 = p_hat[0];
        let mut p1 = p_hat[1];

        for j in (0..d).step_by(2) {
            let zeta = F3329::from_int(ZETAS_INV_128[((2 * byte_rev(j / 2) + 1) * i) % 128]);
            let c0 = p_hat[j].mul(&zeta);
            let c1 = p_hat[j + 1].mul(&zeta);

            p0 = p0.add(&c0);
            p1 = p1.add(&c1);
        }
        a[i] = p0.div(&coeff).unwrap();
        a[i + 1] = p1.div(&coeff).unwrap();
    }

    a
}
