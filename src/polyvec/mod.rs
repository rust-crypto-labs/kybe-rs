//! Polyvec
//!
//! Definiton of basic algebraic structures (Ring, Field, Polynomial, Vector, Matrix)
pub mod structures;

mod matrix;
mod polynomial;
mod polyvec;

pub use matrix::Matrix;
pub use polynomial::Polynomial;
pub use polyvec::PolyVec;
