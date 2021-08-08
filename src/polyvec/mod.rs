//! Polyvec
//!
//! Definiton of basic algebraic structures (Ring, Field, Polynomial, Vector, Matrix)
pub mod structures;

mod matrix;
mod polynomial;
mod polyvec;
mod primefield;

use matrix::Matrix;
use polynomial::Polynomial;
use polyvec::PolyVec;
use primefield::PrimeField3329;

/// Finitefield Z_q
pub type F3329 = PrimeField3329;

/// Polynomial Ring R_q = Z_q[X]/(X^n+1)
pub type Poly3329<const N: usize> = Polynomial<F3329, N>;

/// Polynomial vector R_q^k
pub type PolyVec3329<const N: usize, const D: usize> = PolyVec<Poly3329<N>, D>;

/// Polynomial matrix R_q^(k*k)
pub type PolyMatrix3329<const N: usize, const X: usize, const Y: usize> = Matrix<Poly3329<N>, X, Y>;
