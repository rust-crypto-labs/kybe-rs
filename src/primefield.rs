use crate::polyvec::structures::FiniteField;

use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct PrimeField3329 {
    val: i64,
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
        self.zero().sub(self)
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            val: (self.val + other.val) % (Self::order() as i64),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        let o = Self::order() as i64;

        Self {
            val: (self.val - other.val + o) % o,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            val: (&self.val * &other.val) % (Self::order() as i64),
        }
    }

    fn inv(&self) -> Result<Self, String> {
        if self.is_zero() {
            return Err("DIV0".to_string());
        }
        let mut val = self.one();

        for _i in 1..(Self::order() - 1) {
            val = val.mul(&self)
        }

        Ok(val)
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
    const fn order() -> usize {
        3329
    }

    pub const fn from_int(x: i64) -> Self {
        let o = Self::order() as i64;
        Self { val: (x + o) % o }
    }

    pub const fn to_int(&self) -> i64 {
        self.val
    }
}
