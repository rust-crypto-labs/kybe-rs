//! Key Encapsulation Module
//!
//! Structure that handles all the parameters and functions required to perform the KEM

use crate::functions::utils::{g, h, kdf};
use crate::pke::PKE;
use crate::structures::ByteArray;

pub struct KEM<const N: usize, const K: usize> {
    pke: PKE<N, K>,
    delta: usize,
    pk_size: usize,
    sk_size: usize,
    ct_size: usize,
}

impl<const N: usize, const K: usize> KEM<N, K> {
    /// Kyber CCAKEM Key Generation => (secret key, public key)
    /// Algorithm 7 p. 11
    pub fn keygen(&self) -> (ByteArray, ByteArray) {
        let z = ByteArray::random(32);

        let (sk_prime, pk) = self.pke.keygen();
        let (h1, h2) = h(&pk);
        let sk = ByteArray::concat(&[&sk_prime, &pk, &h1, &h2, &z]);

        (sk, pk)
    }

    /// Encryption : public key  => ciphertext, Shared Key
    /// Algorithm 8 p. 11
    pub fn encaps(&self, pk: &ByteArray) -> (ByteArray, ByteArray) {
        let m = ByteArray::random(32);
        let (m1, m2) = h(&m);
        let (h1, h2) = h(pk);
        let (k_bar, r) = g(&ByteArray::concat(&[&m1, &m2, &h1, &h2]));

        let c = self.pke.encrypt(pk, &m1.append(&m2), r);

        let (h1, h2) = h(&c);
        let k = kdf(&ByteArray::concat(&[&k_bar, &h1, &h2]), self.sk_size);

        (c, k)
    }

    /// Decryption : secret key, ciphertext => Shared Key
    /// Algorithm 9 p. 11
    pub fn decaps(&self, c: &ByteArray, sk: &ByteArray) -> ByteArray {
        // Spliting sk = (sk'||pk||H(pk)||z)
        let (sk_prime, rem) = sk.split_at(12 * K * N / 8);
        let (pk, rem) = rem.split_at(12 * K * N / 8 + 32);
        let (hash, z) = rem.split_at(32);

        let m = self.pke.decrypt(&sk_prime, c);
        let (k_bar, r) = g(&m.append(&hash));
        let c_prime = self.pke.encrypt(&pk, &m, r);

        let (h1, h2) = h(c);
        if *c == c_prime {
            kdf(&ByteArray::concat(&[&k_bar, &h1, &h2]), self.sk_size)
        } else {
            kdf(&ByteArray::concat(&[&z, &h1, &h2]), self.sk_size)
        }
    }

    pub const fn init(
        pke: PKE<N, K>,
        delta: usize,
        pk_size: usize,
        sk_size: usize,
        ct_size: usize,
    ) -> Self {
        Self {
            pke,
            delta,
            pk_size,
            sk_size,
            ct_size,
        }
    }
}

#[test]
fn kem_keygen_ccakem_512() {
    let kem = crate::kyber512kem();
    kem.keygen();
}

#[test]
fn kem_keygen_ccakem_768() {
    let kem = crate::kyber768kem();
    kem.keygen();
}

#[test]
fn encapsulate_then_decapsulate_ccakem_512() {
    let kem = crate::kyber512kem();

    let (sk, pk) = kem.keygen();
    let (ctx, shk) = kem.encaps(&pk);
    let shk2 = kem.decaps(&ctx, &sk);
    assert_eq!(shk, shk2);
}

#[test]
fn encapsulate_then_decapsulate_ccakem_768() {
    let kem = crate::kyber768kem();

    let (sk, pk) = kem.keygen();
    let (ctx, shk) = kem.encaps(&pk);
    let shk2 = kem.decaps(&ctx, &sk);
    assert_eq!(shk, shk2);
}
