//! Polyvec
//!
//! Polynomial vector definition

use crate::structures::algebraics::{FiniteRing, RingModule};

/// Polyvec
#[derive(Clone, Copy)]
pub struct PolyVec<T: FiniteRing, const D: usize> {
    /// Vector coefficients
    pub coefficients: [T; D],
}

impl<T, const D: usize> RingModule<T> for PolyVec<T, D>
where
    T: FiniteRing + Clone + Default + Copy,
{
    fn get(&self, position: usize) -> T {
        self.coefficients[position]
    }

    fn set(&mut self, position: usize, value: T) {
        self.coefficients[position] = value;
    }

    fn zero() -> Self {
        Self::init()
    }

    fn basis_vector(position: usize) -> Self {
        let mut v = Self::zero();
        v.coefficients[position] = T::one();

        v
    }

    fn init() -> Self {
        Self {
            coefficients: [T::zero(); D],
        }
    }

    fn is_zero(&self) -> bool {
        D == 0 || self.coefficients.iter().all(|c| c.is_zero())
    }

    fn neg(&self) -> Self {
        Self::init().sub(self)
    }

    fn dimension() -> usize {
        D
    }

    fn add(&self, other: &Self) -> Self {
        let mut v = [Default::default(); D];

        for (i, el) in v.iter_mut().enumerate() {
            *el = self.coefficients[i].add(&other.coefficients[i]);
        }
        Self::from_vec(v)
    }

    fn sub(&self, other: &Self) -> Self {
        let mut v = [Default::default(); D];

        for (i, el) in v.iter_mut().enumerate() {
            *el = self.coefficients[i].sub(&other.coefficients[i])
        }
        Self::from_vec(v)
    }

    fn dot(&self, other: &Self) -> T {
        let mut v = T::zero();

        for i in 0..D {
            v = v.add(&self.coefficients[i].mul(&other.coefficients[i]))
        }
        v
    }

    fn mulf(&self, other: &T) -> Self {
        let mut v = [Default::default(); D];

        for (i, el) in v.iter_mut().enumerate() {
            *el = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }
}

impl<T, const D: usize> Default for PolyVec<T, D>
where
    T: FiniteRing + Copy,
{
    fn default() -> Self {
        Self {
            coefficients: [T::zero(); D],
        }
    }
}

impl<T, const D: usize> PolyVec<T, D>
where
    T: FiniteRing + Clone + Default,
{
    pub fn from_vec(coefficients: [T; D]) -> Self {
        Self { coefficients }
    }
}
