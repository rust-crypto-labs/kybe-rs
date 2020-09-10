use crate::polyvec::structures::FiniteField;

use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct PrimeField3329 {
    val: usize,
}

impl Debug for PrimeField3329 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

impl FiniteField for PrimeField3329 {
    fn dimension(&self) -> usize {
        1
    }
    fn is_zero(&self) -> bool {
        self.val == 0
    }

    fn zero(&self) -> Self {
        Self { val: 0 }
    }

    fn one(&self) -> Self {
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

impl Default for PrimeField3329 {
    fn default() -> Self {
        Self { val: 0 }
    }
}

impl PrimeField3329 {
    #[inline]
    fn order() -> usize {
        3329
    }

    pub fn from_int(x: usize) -> Self {
        Self { val: x }
    }
}
