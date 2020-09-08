use crate::polyvec::ff::FiniteRing;
use crate::polyvec::vector::{Dot, Vector};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Polyvec<T: FiniteRing> {
    coefficients: Vec<T>,
    dimension: usize,
}

impl<T> Vector for Polyvec<T>
where
    T: FiniteRing + Clone + Default,
{
    fn basis_vector(&self, position: usize) -> Self {
        assert!(position < self.dimension());

        let t: T = Default::default();
        let mut coefficients = vec![t.zero(); self.dimension()];
        coefficients[position] = t.one();

        Self {
            coefficients,
            dimension: self.dimension(),
        }
    }

    fn init(dimension: usize) -> Self {
        let t: T = Default::default();
        Self {
            coefficients: vec![t.zero(); dimension],
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

impl<T> Dot<T> for Polyvec<T>
where
    T: FiniteRing + Clone + Default,
{
    fn dot(&self, other: &Self) -> T {
        assert_eq!(self.dimension(), other.dimension());
        let t: T = Default::default();
        let mut v = t.zero();

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

impl<T> Default for Polyvec<T>
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

impl<T> Polyvec<T>
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

    pub fn mulf(&self, other: &T) -> Self {
        let mut v = vec![];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }
}
