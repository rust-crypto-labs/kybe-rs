//! This is documentation for the `kybe-rs` crate.
//!
//! # Introduction
//! `kybe-rs` is an implementation of Crystals-Kyber , a post-quantum
//! candidate submitted to NIST for standardization.
//!
//! This crate provides public-key encryption (`PKE`) and key encapsulation (`KEM`).
//!
//! # Examples
//!
//! ```rust
//! use kybe_rs::{self, KyberParams};
//! let params = KyberParams::kyber512();
//!
//! // Alice runs keygen, publishes pk. Value sk is secret
//! let (sk, pk) = kybe_rs::kyber_ccakem_key_gen(params);
//!
//! // Bob uses pk3 to derive a key k and encapsulation c
//! let (c, k) = kybe_rs::kyber_ccakem_enc(params, &pk);
//!
//! // Bob sends c to Alice
//! // Alice uses s, c, sk3 and pk3 to recover k
//! let k_recovered = kybe_rs::kyber_ccakem_dec(params, &c, &sk);
//!
//! assert_eq!(k, k_recovered);
//! ```

extern crate sha3;

mod bytearray;
mod compress;
mod encode;
mod hash;
mod ntt;
mod params;
mod polyvec;
mod primefield;
mod utils;

use ntt::{bcm_matrix_vec, ntt_product_matvec, ntt_product_vec, ntt_vec};
use polyvec::{
    structures::{FiniteRing, RingModule},
    Matrix, PolyVec, Polynomial,
};
use primefield::PrimeField3329;
use utils::{cbd, g, h, kdf, parse, prf, xof};

pub use bytearray::ByteArray;
pub use compress::{compress_poly, compress_polyvec, decompress_poly, decompress_polyvec};
pub use encode::{decode_to_poly, decode_to_polyvec, encode_poly, encode_polyvec};
pub use params::KyberParams;

/// Finitefield Z_q
pub type F3329 = PrimeField3329;

/// Polynomial Ring R_q = Z_q[X]/(X^n+1)
pub type Poly3329<const N: usize> = Polynomial<F3329, N>;

/// Polynomial vector R_q^k
pub type PolyVec3329<const N: usize, const D: usize> = PolyVec<Poly3329<N>, D>;

/// Polynomial matrix R_q^(k*k)
pub type PolyMatrix3329<const N: usize, const X: usize, const Y: usize> = Matrix<Poly3329<N>, X, Y>;

/// Default length used for XOF
const XOF_LEN: usize = 4000;

/// Kyber CPAPKE Key Generation => (secret key, public key)
/// Algorithm 4 p. 9
pub fn kyber_cpapke_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let k = params.k;
    let d = ByteArray::random(32);
    let (rho, sigma) = g(&d);

    let mut a = PolyMatrix3329::init_matrix(k, k);

    for i in 0..k {
        for j in 0..k {
            a.set(i, j, parse(&xof(&rho, j, i, XOF_LEN), params.q));
        }
    }

    let (mut s, mut e) = (PolyVec3329::<256, 2>::init(), PolyVec3329::<256, 2>::init());
    let prf_len = 64 * params.eta;

    for i in 0..k {
        s.set(i, cbd(prf(&sigma, i, prf_len), params.eta));
        e.set(i, cbd::<256>(prf(&sigma, k + i, prf_len), params.eta));
    }
    let s_hat = ntt_vec(&s);
    let e_hat = ntt_vec(&e);

    let t_hat = bcm_matrix_vec(&a, &s_hat).add(&e_hat);

    let pk = encode_polyvec(t_hat, 12).append(&rho);
    let sk = encode_polyvec(s_hat, 12);

    (sk, pk)
}

/// Kyber CPAPKE Encryption : public key, message, random coins => ciphertext
/// Algorithm 5 p. 10
pub fn kyber_cpapke_enc(
    params: KyberParams,
    pk: &ByteArray,
    m: &ByteArray,
    r: ByteArray,
) -> ByteArray {
    let offset = 12 * params.k * params.n / 8;
    let prf_len = 64 * params.eta;

    let (t, rho) = pk.split_at(offset);
    let t_hat = decode_to_polyvec(t, 12);
    let mut a_t = PolyMatrix3329::init_matrix(params.k, params.k);

    for i in 0..params.k {
        for j in 0..params.k {
            a_t.set(i, j, parse(&xof(&rho, i, j, XOF_LEN), params.q));
        }
    }

    let (mut r_bold, mut e1) = (PolyVec3329::<256, 2>::init(), PolyVec3329::<256, 2>::init());
    for i in 0..params.k {
        r_bold.set(i, cbd(prf(&r, i, prf_len), params.eta));
        e1.set(i, cbd(prf(&r, params.k + i, prf_len), params.eta));
    }
    let e2 = cbd(prf(&r, 2 * params.k, prf_len), params.eta);

    let r_hat = ntt_vec(&r_bold);
    let u_bold = ntt_product_matvec(&a_t, &r_hat).add(&e1);

    let v = ntt_product_vec(&t_hat, &r_hat)
        .add(&e2)
        .add(&decompress_poly(
            decode_to_poly::<256>(m.clone(), 1),
            1,
            params.q,
        ));

    let c1 = encode_polyvec(compress_polyvec(u_bold, params.du, params.q), params.du);
    let c2 = encode_poly(compress_poly(v, params.dv, params.q), params.dv);

    c1.append(&c2)
}

/// Kyber CPAPKE Decryption : secret key, ciphertext => message
/// Algorithm 6 p. 10
pub fn kyber_cpapke_dec(params: KyberParams, sk: &ByteArray, c: &ByteArray) -> ByteArray {
    let offset = params.du * params.k * params.n / 8;
    let (c1, c2) = c.split_at(offset);

    let u = decompress_polyvec(
        decode_to_polyvec::<256, 2>(c1, params.du),
        params.du,
        params.q,
    );
    let v = decompress_poly(decode_to_poly(c2, params.dv), params.dv, params.q);
    let s = decode_to_polyvec(sk.clone(), 12);

    let u_hat = ntt_vec(&u);
    let x = ntt_product_vec(&s, &u_hat);
    let p = v.sub(&x);

    encode_poly(compress_poly(p, 1, params.q), 1)
}

/// Kyber CCAKEM Key Generation => (secret key, public key)
/// Algorithm 7 p. 11
pub fn kyber_ccakem_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let z = ByteArray::random(32);

    let (sk_prime, pk) = kyber_cpapke_key_gen(params);
    let (h1, h2) = h(&pk);
    let sk = ByteArray::concat(&[&sk_prime, &pk, &h1, &h2, &z]);

    (sk, pk)
}

/// Encryption : public key  => ciphertext, Shared Key
/// Algorithm 8 p. 11
pub fn kyber_ccakem_enc(params: KyberParams, pk: &ByteArray) -> (ByteArray, ByteArray) {
    let m = ByteArray::random(32);
    let (m1, m2) = h(&m);
    let (h1, h2) = h(pk);
    let (k_bar, r) = g(&ByteArray::concat(&[&m1, &m2, &h1, &h2]));

    let c = kyber_cpapke_enc(params, pk, &m1.append(&m2), r);

    let (h1, h2) = h(&c);
    let k = kdf(&ByteArray::concat(&[&k_bar, &h1, &h2]), params.sk_size);

    (c, k)
}

/// Decryption : secret key, ciphertext => Shared Key
/// Algorithm 9 p. 11
pub fn kyber_ccakem_dec(params: KyberParams, c: &ByteArray, sk: &ByteArray) -> ByteArray {
    // Spliting sk = (sk'||pk||H(pk)||z)
    let (sk_prime, rem) = sk.split_at(12 * params.k * params.n / 8);
    let (pk, rem) = rem.split_at(12 * params.k * params.n / 8 + 32);
    let (hash, z) = rem.split_at(32);

    let m = kyber_cpapke_dec(params, &sk_prime, c);
    let (k_bar, r) = g(&m.append(&hash));
    let c_prime = kyber_cpapke_enc(params, &pk, &m, r);

    let (h1, h2) = h(c);
    if *c == c_prime {
        kdf(&ByteArray::concat(&[&k_bar, &h1, &h2]), params.sk_size)
    } else {
        kdf(&ByteArray::concat(&[&z, &h1, &h2]), params.sk_size)
    }
}
