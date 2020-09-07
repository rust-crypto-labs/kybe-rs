use crate::polyvec::ff::{FiniteField, FiniteRing};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Polynomial<T: FiniteField> {
    coefficients: Vec<T>,
    degree: usize,
}

impl<T> FiniteRing for Polynomial<T>
where
    T: FiniteField + Clone,
{
    fn is_zero(&self) -> bool {
        self.eq(&Self::zero())
    }

    fn dimension() -> usize {
        T::dimension()
    }

    fn zero() -> Self {
        Self {
            coefficients: vec![T::zero()],
            degree: 0,
        }
    }

    fn one() -> Self {
        Self {
            coefficients: vec![T::one()],
            degree: 0,
        }
    }

    fn neg(&self) -> Self {
        let degree = self.degree();
        let mut coefficients = vec![T::zero(); degree];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = c.neg();
        }
        Self {
            coefficients,
            degree,
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut degree = self.degree().max(other.degree);
        let mut coefficients = vec![T::zero(); degree];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = other.coefficients[i].add(c);
        }

        // Diminish degree if leading coefficient is zero
        while degree > 0 && coefficients[degree - 1].eq(&T::zero()) {
            degree -= 1;
        }

        Self {
            coefficients,
            degree,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// MOD X^N + 1  !
    fn mul(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    fn into_bytes(self) -> Vec<u8> {
        unimplemented!()
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}
impl<T> PartialEq for Polynomial<T>
where
    T: FiniteField,
{
    fn eq(&self, other: &Self) -> bool {
        if self.degree != other.degree {
            return false;
        }

        for (i, c) in self.coefficients.iter().enumerate() {
            if !c.eq(&other.coefficients[i]) {
                return false;
            }
        }
        true
    }
}

impl<T> Eq for Polynomial<T> where T: FiniteField {}

impl<T> Polynomial<T>
where
    T: FiniteField + Clone,
{
    pub fn from_vec(coefficients: Vec<T>) -> Self {
        let degree = coefficients.len();
        Self {
            coefficients,
            degree,
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn mulf(&self, other: &T) -> Self {
        let mut v = vec![];

        for i in 0..self.degree() {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }
}

impl<T> Index<usize> for Polynomial<T>
where
    T: FiniteField,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.coefficients[index]
    }
}

impl<T> IndexMut<usize> for Polynomial<T>
where
    T: FiniteField,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.coefficients[index]
    }
}
