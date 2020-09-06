use crate::polyvec::ff::FiniteRing;
use crate::polyvec::vector::{Dot, Vector};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Polyvec<T: FiniteRing> {
    coefficients: Vec<T>,
    dimension: usize,
}

impl<'a, T> Vector for Polyvec<T>
where
    T: FiniteRing + Clone,
{
    fn basis_vector(&self, position: usize) -> Self {
        assert!(position < self.dimension());

        let mut coefficients = vec![T::zero(); self.dimension()];
        coefficients[position] = T::one();

        Self {
            coefficients,
            dimension: self.dimension(),
        }
    }

    fn init(dimension: usize) -> Self {
        Self {
            coefficients: vec![T::zero(); dimension],
            dimension,
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.dimension(), other.dimension());
        let mut v = vec![];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].add(&other.coefficients[i])
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
}

impl<T> Polyvec<T>
where
    T: FiniteRing + Clone,
{
    pub fn from_vec(coefficients: Vec<T>) -> Self {
        let dimension = coefficients.len();
        Self {
            coefficients,
            dimension,
        }
    }

    pub fn mulf(&self, other: &T) -> Self {
        let mut v = vec![];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }
}

impl<T> Dot<T> for Polyvec<T>
where
    T: FiniteRing + Clone,
{
    fn dot(&self, other: &Self) -> T {
        assert_eq!(self.dimension(), other.dimension());
        let mut v = T::zero();

        for i in 0..self.dimension() {
            v = v.add(&self.coefficients[i].mul(&other.coefficients[i]))
        }
        v
    }
}

impl<T> Index<usize> for Polyvec<T>
where
    T: FiniteRing,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.coefficients[index]
    }
}

impl<T> IndexMut<usize> for Polyvec<T>
where
    T: FiniteRing,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.coefficients[index]
    }
}
