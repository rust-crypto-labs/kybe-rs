mod ff;
pub mod matrix;
pub mod polynomial;
pub mod polyvec;
pub mod vector;

use ff::{FiniteField, FiniteRing};
use matrix::Matrix;
use polynomial::Polynomial;
use polyvec::Polyvec;
use std::fmt::Debug;

pub type Polynom3329 = Polynomial<PrimeField3329>;
pub type Polyvec3329 = Polyvec<Polynom3329>;
pub type Polymatrix3329 = Matrix<Polyvec3329>;

#[derive(Clone)]
pub struct PrimeField3329 {
    val: usize,
}

impl Debug for PrimeField3329 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

impl PrimeField3329 {
    #[inline]
    fn order() -> usize {
        3329
    }
}

impl FiniteRing for PrimeField3329 {
    fn is_zero(&self) -> bool {
        self.val == Self::zero().val
    }

    fn dimension() -> usize {
        1
    }

    fn zero() -> Self {
        Self { val: 0 }
    }

    fn one() -> Self {
        Self { val: 1 }
    }

    fn neg(&self) -> Self {
        Self {
            val: Self::order() - &self.val,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            val: (self.val + other.val) % Self::order(),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            val: (&self.val * &other.val) % Self::order(),
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        unimplemented!()
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl FiniteField for PrimeField3329 {
    fn inv(&self) -> Result<Self, String> {
        unimplemented!()
    }

    fn div(&self, other: &Self) -> Result<Self, String> {
        Ok(self.mul(&other.inv()?))
    }
}

impl PartialEq for PrimeField3329 {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for PrimeField3329 {}
