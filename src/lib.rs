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

mod functions;
mod kem;
mod pke;
mod structures;

pub use structures::ByteArray;

use kem::KEM;
use pke::PKE;

pub const fn kyber512pke() -> PKE<256, 2> {
    PKE::<256, 2>::init(3329, 2, 10, 3)
}

pub const fn kyber512kem() -> KEM<256, 2> {
    KEM::<256, 2>::init(kyber512pke(), 178, 800, 1632, 738)
}

pub const fn kyber768pke() -> PKE<256, 3> {
    PKE::<256, 3>::init(3329, 2, 10, 4)
}

pub const fn kyber768kem() -> KEM<256, 3> {
    KEM::<256, 3>::init(kyber768pke(), 164, 1184, 2400, 1088)
}
