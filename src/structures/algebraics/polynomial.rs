//! Polynomials
//!
//! Polynomial structure

use crate::structures::algebraics::{FiniteField, FiniteRing};

use std::ops::Index;

/// Represents a polynomial in the ring T[X]/(X^n + 1)
#[derive(Clone, Copy)]
pub struct Polynomial<T, const N: usize>
where
    T: FiniteField + Default,
{
    /// Coefficients of the polynomial
    pub coefficients: [T; N],

    /// Degree of the polynomial (the zero polynomial has degree < 0)
    pub degree: Option<usize>,
}

impl<T, const N: usize> FiniteRing for Polynomial<T, N>
where
    T: FiniteField + Clone + Default + Copy,
{
    fn is_zero(&self) -> bool {
        self.degree().is_none()
    }

    fn zero() -> Self {
        Self {
            coefficients: [T::zero(); N],
            degree: None,
        }
    }

    fn one() -> Self {
        let mut p = Self::zero();
        p.set_coeff(0, T::one());
        p
    }

    fn neg(&self) -> Self {
        // If the polynomial is already zero, do nothing
        if self.is_zero() {
            return Self::zero();
        }

        let mut coefficients = [T::zero(); N];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = c.neg();
        }
        Self {
            coefficients,
            degree: self.degree,
        }
    }

    fn add(&self, other: &Self) -> Self {
        // If one of the polynomial is already zero, do nothing
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        // Unwraps safely since the case None has been tested above
        let mut degree: usize = self.degree().unwrap().max(other.degree().unwrap());

        let mut coefficients = [T::zero(); N];
        for (i, el) in coefficients.iter_mut().enumerate() {
            *el = self[i].add(&other[i]);
        }

        // Diminish degree if leading coefficient is zero
        let mut leading = &coefficients[degree];
        while degree > 0 && leading.eq(&T::zero()) {
            degree -= 1;
            leading = &coefficients[degree];
        }

        // Check whether the result is zero
        if degree == 0 && leading.eq(&T::zero()) {
            return Self::zero();
        }

        Self {
            coefficients,
            degree: Some(degree),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let coeffs = [T::zero(); N];

        for i in 0..N {
            for j in 0..N {
                let c = self[i].mul(&other[j]);
                let k = i + j;
                if k < N {
                    coeffs[k].add(&c);
                } else {
                    // X^n = -1
                    coeffs[k % N].sub(&c);
                }
            }
        }

        // Reduce degree if appropriate
        let mut degree = N - 1;
        while degree > 0 && coeffs[degree].eq(&T::zero()) {
            degree -= 1;
        }

        // Check for null polynomial (shouldn't happen but still)
        if degree == 0 && coeffs[0].eq(&T::zero()) {
            return Self::zero();
        }

        Self {
            coefficients: coeffs,
            degree: Some(degree),
        }
    }
}

impl<T, const N: usize> PartialEq for Polynomial<T, N>
where
    T: FiniteField + Default,
{
    fn eq(&self, other: &Self) -> bool {
        if self.degree != other.degree {
            return false;
        }

        for i in 0..N {
            if !self[i].eq(&other[i]) {
                return false;
            }
        }
        true
    }
}

impl<T, const N: usize> Eq for Polynomial<T, N> where T: FiniteField + Default {}

impl<T, const N: usize> Polynomial<T, N>
where
    T: FiniteField + Clone + Default + Copy,
{
    /// Init polynomial with a default value
    pub fn init() -> Self {
        Self::from_vec([Default::default(); N])
    }

    /// Return dimension of the Rq module
    pub fn dimension() -> usize {
        N
    }

    /// Init polynomial with specified coefficients
    /// If the array is bigger than N, only the first N values are taken
    pub fn from_vec(coefficients: [T; N]) -> Self {
        // Reduce degree if appropriate
        let mut degree = N - 1;
        while degree > 0 && coefficients[degree].eq(&T::zero()) {
            degree -= 1;
        }

        // Check for null polynomial (shouldn't happen but still)
        if degree == 0 && coefficients[0].eq(&T::zero()) {
            return Self::zero();
        }

        Self {
            coefficients,
            degree: Some(degree),
        }
    }

    /// Return polynomial degree
    pub fn degree(&self) -> Option<usize> {
        self.degree
    }

    /// Multiplication by a scalar
    pub fn mulf(&self, other: &T) -> Self {
        // If the polynomial or the scalar is already zero, do nothing
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        // Unwraps safely since the case None has been tested above
        let degree = self.degree().unwrap();

        let mut v = [Default::default(); N];

        for (i, el) in v.iter_mut().enumerate().take(degree) {
            *el = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }

    /// Set a coefficient of the polynomial, recalculates the degree
    /// Ignores values beyond the dimension of the polynomial
    pub fn set_coeff(&mut self, index: usize, val: T) {
        if index < N && !val.is_zero() {
            self.degree = match self.degree() {
                Some(d) if d < index => Some(index),
                None => Some(index),
                Some(d) => Some(d),
            };

            self.coefficients[index] = val;
        }
    }
}

impl<T, const N: usize> Index<usize> for Polynomial<T, N>
where
    T: FiniteField + Default,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.coefficients[index]
    }
}

impl<T, const N: usize> Default for Polynomial<T, N>
where
    T: FiniteField + Clone + Default + Copy,
{
    fn default() -> Self {
        Self::init()
    }
}
