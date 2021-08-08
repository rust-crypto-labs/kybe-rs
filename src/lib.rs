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
//! For the KEM:
//!
//! ```rust
//! use kybe_rs::kyber512kem;
//! let kem = kyber512kem();
//!
//! // Alice runs keygen, publishes pk. Value sk is secret
//! let (sk, pk) = kem.keygen();
//!
//! // Bob uses pk3 to derive a key k and encapsulation c
//! let (c, k) = kem.encaps(&pk);
//!
//! // Bob sends c to Alice
//! // Alice uses s, c, sk3 and pk3 to recover k
//! let k_recovered = kem.decaps(&c, &sk);
//!
//! assert_eq!(k, k_recovered);
//! ```
//! For the PKE:
//!
//! ```rust
//! use kybe_rs::{kyber512pke, ByteArray};
//! let pke = kyber512pke();
//!
//! // Bob wants to send an encrypted message to Alice
//! let m = ByteArray::random(32);
//! let r = ByteArray::random(32);
//!
//! // Alice runs keygen, publishes pk. Value sk is secret
//! let (sk, pk) = pke.keygen();
//!
//! // Bob uses the public key to encrypt the message
//! let enc = pke.encrypt(&pk, &m, r.clone());
//!
//! // Bob sends enc to Alice
//! // Alice uses the secret key to recover m
//! let dec = pke.decrypt(&sk, &enc);
//!
//! assert_eq!(m, dec);
//! ```

extern crate sha3;

mod functions;
mod kem;
mod pke;
mod structures;

pub use structures::ByteArray;

use kem::KEM;
use pke::PKE;

/// Instantiate the Kyber 512 PKE with the appropriate parameters
pub const fn kyber512pke() -> PKE<256, 2> {
    PKE::<256, 2>::init(3329, 2, 10, 3)
}

/// Instantiate the Kyber 512 KEM with the appropriate parameters
pub const fn kyber512kem() -> KEM<256, 2> {
    KEM::<256, 2>::init(kyber512pke(), 178, 800, 1632, 738)
}

/// Instantiate the Kyber 768 PKE with the appropriate parameters
pub const fn kyber768pke() -> PKE<256, 3> {
    PKE::<256, 3>::init(3329, 2, 10, 4)
}

/// Instantiate the Kyber 768 KEM with the appropriate parameters
pub const fn kyber768kem() -> KEM<256, 3> {
    KEM::<256, 3>::init(kyber768pke(), 164, 1184, 2400, 1088)
}
