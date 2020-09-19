extern crate sha3;

use std::convert::TryInto;

mod bytearray;
mod compress;
mod encode;
mod hash;
mod ntt;
mod params;
mod polyvec;
mod primefield;

use compress::{compress_polyvec, decompress_polyvec};
use ntt::{bcm_matrix_vec, ntt_product_matvec, ntt_product_vec, ntt_vec};
use polyvec::{structures::RingModule, Matrix, PolyVec, Polynomial};
use primefield::PrimeField3329;

pub use bytearray::ByteArray;
pub use encode::{decode_to_poly, decode_to_polyvec, encode_poly, encode_polyvec};
pub use params::KyberParams;

pub type F3329 = PrimeField3329;
pub type Poly3329 = Polynomial<F3329>;
pub type PolyVec3329 = PolyVec<Poly3329>;
pub type PolyMatrix3329 = Matrix<Poly3329>;

const XOF_LEN: usize = 4000;
////////////// PKE /////////////////////////

// Kyber CPAPKE Key Generation => (secret key, public key)
// Algorithm 4 p. 9
pub fn kyber_cpapke_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let k = params.k;
    let d = ByteArray::random(32);
    let (rho, sigma) = g(d);

    let mut a = PolyMatrix3329::init_matrix(k, k);

    for i in 0..k {
        for j in 0..k {
            a.set(i, j, parse(xof(&rho, j, i, XOF_LEN), params.n, params.q));
        }
    }

    let (mut s, mut e) = (PolyVec3329::init(256), PolyVec3329::init(256));
    let prf_len = 64 * params.eta;

    for i in 0..k {
        s.set(i, cbd(prf(&sigma, i, prf_len), params.eta));
        e.set(i, cbd(prf(&sigma, k + i, prf_len), params.eta));
    }
    let s_hat = ntt_vec(&s);
    let e_hat = ntt_vec(&e);

    let t_hat = bcm_matrix_vec(&a, &s_hat).add(&e_hat);

    // TODO: mod+ q  ?
    let sk = encode_polyvec(t_hat).append(&rho);
    let pk = encode_polyvec(s_hat);

    (sk, pk)
}

// Encryption : public key, message, random coins => ciphertext
pub fn kyber_cpapke_enc(
    params: KyberParams,
    pk: &ByteArray,
    m: &ByteArray,
    r: ByteArray,
) -> ByteArray {
    let mut n_ctr = 0;
    let offset = 12 * params.k * params.n / 8;
    let prf_len = 64 * params.eta;

    let t_hat = decode_to_polyvec(pk);
    let rho = pk.skip(offset);
    let mut a = PolyMatrix3329::init_matrix(params.k, params.k);

    for i in 0..params.k {
        for j in 0..params.k {
            a.set(i, j, parse(xof(&rho, j, i, XOF_LEN), params.n, params.q));
        }
    }

    let mut r_bold = vec![];
    for _ in 0..params.k {
        r_bold.push(cbd(prf(&r, n_ctr, prf_len), params.eta));
        n_ctr += 1;
    }
    let r_bold = PolyVec3329::from_vec(r_bold);

    let mut e1 = vec![];
    for _ in 0..params.k {
        e1.push(cbd(prf(&r, n_ctr, prf_len), params.eta));
        n_ctr += 1;
    }
    let e1 = PolyVec3329::from_vec(e1);
    let e2 = PolyVec3329::from_vec(vec![cbd(prf(&r, n_ctr, prf_len), params.eta)]);

    let r_hat = ntt_vec(&r_bold);
    let u_bold = ntt_product_matvec(&a, &r_hat).add(&e1);

    let v = ntt_product_vec(&t_hat, &r_hat)
        .add(&e2)
        .add(&decompress_polyvec(decode_to_polyvec(m), 1, params.q));

    let c1 = encode_polyvec(compress_polyvec(u_bold, params.du, params.q));
    let c2 = encode_polyvec(compress_polyvec(v, params.dv, params.q));

    c1.append(&c2)
}

// Decryption : secret key, ciphertext => message
pub fn kyber_cpapke_dec(params: KyberParams, sk: &ByteArray, c: &ByteArray) -> ByteArray {
    let u = decompress_polyvec(
        decode_to_polyvec(&c.truncate(params.du)),
        params.du,
        params.q,
    );

    let offset = params.du * params.k * params.n / 8;

    let v = decompress_polyvec(
        decode_to_polyvec(&c.skip(offset).truncate(params.dv)),
        params.dv,
        params.q,
    );

    let u_hat = ntt_vec(&u);
    let s = decode_to_polyvec(sk);

    encode_polyvec(compress_polyvec(
        v.sub(&ntt_product_vec(&s, &u_hat)),
        1,
        params.q,
    ))
}

////////////// KEM /////////////////////////

// Kyber CCAKEM Key Generation => (secret key, public key)
// Algorithm 7 p. 11
pub fn kyber_ccakem_key_gen(params: KyberParams) -> (ByteArray, ByteArray) {
    let z = ByteArray::random(32);
    let (pk, sk) = kyber_cpapke_key_gen(params);
    let (h1, h2) = h(&pk);
    let sk = ByteArray::concat(&[&sk, &pk, &h1, &h2, &z]);
    (sk, pk)
}

// Encryption : public key  => ciphertext, Shared Key
// Algorithm 8 p. 11
pub fn kyber_ccakem_enc(params: KyberParams, pk: &ByteArray) -> (ByteArray, ByteArray) {
    let m = ByteArray::random(32);
    let (m1, m2) = h(&m);
    let (h1, h2) = h(pk);
    let (k, r) = g(ByteArray::concat(&[&m1, &m2, &h1, &h2]));
    let c = kyber_cpapke_enc(params, pk, &m1.append(&m2), r);
    let (h1, h2) = h(&c);
    let k = kdf(&ByteArray::concat(&[&k, &h1, &h2]), params.sk_size);

    (c, k)
}

// Decryption : secret key, ciphertext => Shared Key
pub fn kyber_ccakem_dec(params: KyberParams, c: &ByteArray, sk: &ByteArray) -> ByteArray {
    let pk = sk.skip(12 * params.k * params.n / 8);
    let hash = sk.skip(24 * params.k * params.n / 8 + 32).truncate(32);
    let z = sk.skip(24 * params.k * params.n / 8 + 64);

    let m = kyber_cpapke_dec(params, sk, c);
    let (k, r) = g(m.append(&hash));
    let c_prime = kyber_cpapke_enc(params, &pk, &m, r);

    let (h1, h2) = h(c);
    if *c == c_prime {
        kdf(&ByteArray::concat(&[&k, &h1, &h2]), params.sk_size)
    } else {
        kdf(&ByteArray::concat(&[&z, &h1, &h2]), params.sk_size)
    }
}

////////////////// Utils ////////////////////

// receives as input a byte stream B=(b0; b1; b2;...) and computes the NTT-representation a' = a'_0 + a'_0X + ... + a'_n-1X^(n-1) in R_q of a in R_q
// Algorithm 1 p. 7
fn parse(bs: ByteArray, n: usize, q: usize) -> Poly3329 {
    let mut i = 0;
    let mut j = 0;
    let mut coeffs = vec![F3329::default(); n];
    while j < n {
        let d = (bs.data[i] as usize) + (bs.data[i + 1] as usize) << 8;
        if d < 19 * q {
            coeffs[j] = F3329::from_int(d.try_into().unwrap());
            j += 1;
        }
        i += 2;
    }
    Poly3329::from_vec(coeffs, n)
}

// Centered Binomial Distribution
// Algorithm 2 p. 8
// Takes as input an array of 64 eta bytes
fn cbd(bs: ByteArray, eta: usize) -> Poly3329 {
    let mut f_coeffs = vec![F3329::default(); 256];
    for i in 0..256 {
        let mut a = 0;
        let mut b = 0;

        for j in 0..eta {
            if bs.get_bit(2 * i * eta + j) {
                a += 1;
            }
            if bs.get_bit(2 * i * eta + eta + j) {
                b += 1;
            }
        }

        f_coeffs[i] = F3329::from_int(a - b);
    }
    Poly3329::from_vec(f_coeffs, 256)
}

// Pseudo random function => SHAKE-256(s||b);
fn prf(s: &ByteArray, b: usize, len: usize) -> ByteArray {
    let b_as_bytes = ByteArray {
        data: (b as u64).to_be_bytes().to_vec(),
    };
    let input = ByteArray::concat(&[s, &b_as_bytes]);
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

    let input = ByteArray::concat(&[r, &j_as_bytes, &i_as_bytes]);
    ByteArray {
        data: hash::shake_128(input.data, len),
    }
}

// Hash function => SHA3-256
fn h(r: &ByteArray) -> (ByteArray, ByteArray) {
    let hash = hash::sha3_256(r.data.clone());
    let (part0, part1) = hash.split_at(16);

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
    let hash = hash::sha3_512(r.data);
    let (part0, part1) = hash.split_at(32);

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
fn kdf(r: &ByteArray, len: usize) -> ByteArray {
    let hash = hash::shake_256(r.data.clone(), len);

    ByteArray { data: hash }
}
