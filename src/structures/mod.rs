//! Structures
//!
//! Definiton of all structures used across the crate

mod bytearray;
mod primefield;

pub mod algebraics;

use algebraics::Matrix;
use algebraics::PolyVec;
use algebraics::Polynomial;
use primefield::PrimeField3329;

pub use bytearray::ByteArray;

/// Finitefield Z_q
pub type F3329 = PrimeField3329;

/// Polynomial Ring R_q = Z_q[X]/(X^n+1)
pub type Poly3329<const N: usize> = Polynomial<F3329, N>;

/// Polynomial vector R_q^k
pub type PolyVec3329<const N: usize, const D: usize> = PolyVec<Poly3329<N>, D>;

/// Polynomial matrix R_q^(k*k)
pub type PolyMatrix3329<const N: usize, const X: usize, const Y: usize> = Matrix<Poly3329<N>, X, Y>;
