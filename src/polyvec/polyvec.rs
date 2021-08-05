//! Polyvec
//!
//! Polynomial vector definition

use crate::polyvec::structures::{FiniteRing, RingModule};

/// Polyvec
#[derive(Clone)]
pub struct PolyVec<T: FiniteRing, const D: usize> {
    /// Vector coefficients
    pub coefficients: Vec<T>,

    /// Size of vector
    pub dimension: usize,
}

impl<T, const D: usize> RingModule<T> for PolyVec<T,D>
where
    T: FiniteRing + Clone + Default,
{
    fn get(&self, position: usize) -> T {
        self.coefficients[position].clone()
    }

    fn set(&mut self, position: usize, value: T) {
        self.coefficients[position] = value;
    }

    fn zero() -> Self {
        Self::init()
    }

    fn basis_vector(position: usize) -> Self {
        let mut coefficients = vec![T::zero(); D];
        coefficients[position] = T::one();

        Self {
            coefficients,
            dimension: D,
        }
    }

    fn init() -> Self {
        Self {
            coefficients: vec![T::zero(); D],
            dimension: D,
        }
    }

    fn is_zero(&self) -> bool {
        if self.dimension == 0 {
            true
        } else {
            self.coefficients.iter().all(|c| c.is_zero())
        }
    }

    fn neg(&self) -> Self {
        Self::init().sub(self)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.dimension(), other.dimension());
        let mut v = vec![Default::default(); self.dimension()];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].add(&other.coefficients[i]);
        }
        Self::from_vec(v)
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.dimension(), other.dimension());
        let mut v = vec![];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].sub(&other.coefficients[i])
        }
        Self::from_vec(v)
    }

    fn dot(&self, other: &Self) -> T {
        assert_eq!(self.dimension(), other.dimension());
        let mut v = T::zero();

        for i in 0..self.dimension() {
            v = v.add(&self.coefficients[i].mul(&other.coefficients[i]))
        }
        v
    }

    fn mulf(&self, other: &T) -> Self {
        let mut v = vec![];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }
}

impl<T, const D: usize> Default for PolyVec<T, D>
where
    T: FiniteRing,
{
    fn default() -> Self {
        Self {
            coefficients: vec![],
            dimension: 0,
        }
    }
}

impl<T, const D: usize> PolyVec<T, D>
where
    T: FiniteRing + Clone + Default,
{
    pub fn from_vec(coefficients: Vec<T>) -> Self {
        let dimension = coefficients.len();
        Self {
            coefficients,
            dimension,
        }
    }
}
