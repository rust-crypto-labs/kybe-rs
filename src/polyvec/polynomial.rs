//! Polynomials
//!
//! Polynomial structure

use crate::polyvec::structures::{FiniteField, FiniteRing};
use std::{
    ops::Index
};

/// Represents a polynomial in the ring T[X]/(X^n + 1)
#[derive(Clone)]
pub struct Polynomial<T, const N: usize>
where
    T: FiniteField + Default,
{
    /// Coefficients of the polynomial
    pub coefficients: Vec<T>,

    /// Degree of the polynomial (the zero polynomial has degree < 0)
    pub degree: Option<usize>,
}

impl<T, const N: usize> FiniteRing for Polynomial<T, N>
where
    T: FiniteField + Clone + Default,
{
    fn is_zero(&self) -> bool {
        self.degree().is_none()
    }

    fn zero() -> Self {
        Self {
            coefficients: vec![T::zero()],
            degree: None,
        }
    }

    fn one() -> Self {
        Self {
            coefficients: vec![T::one()],
            degree: Some(0),
        }
    }

    fn neg(&self) -> Self {
        // If the polynomial is already zero, do nothing
        if self.is_zero() {
            return Self::zero();
        }
        // Unwraps safely since the case None has been tested above
        let degree = self.degree().unwrap();

        let mut coefficients = vec![T::zero(); degree + 1];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = c.neg();
        }
        Self {
            coefficients,
            degree: Some(degree),
        }
    }

    fn add(&self, other: &Self) -> Self {
        // If one of the polynomial is already zero, do nothing
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        // Unwraps safely since the case None has been tested above
        let mut degree: usize = self.degree().unwrap().max(other.degree().unwrap());

        let mut coefficients = vec![T::zero(); degree + 1];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = other.coefficients[i].add(c);
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
        if self.is_zero() {
            return self.clone();
        }
        if other.is_zero() {
            return other.clone();
        }
        let coeffs = vec![T::zero(); N];

        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                let c = a.mul(&b);
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

        for (i, c) in self.coefficients.iter().enumerate() {
            if !c.eq(&other.coefficients[i]) {
                return false;
            }
        }
        true
    }
}

impl<T, const N: usize> Eq for Polynomial<T, N> where T: FiniteField + Default {}

impl<T, const N: usize> Polynomial<T, N>
where
    T: FiniteField + Clone + Default,
{
    /// Init polynomial with a default value
    pub fn init() -> Self {
        Self::from_vec(vec![Default::default(); N])
    }

    /// Return dimension of the Rq module
    pub fn dimension() -> usize {
        N
    }

    /// Init polynomial with specified coefficients
    pub fn from_vec(mut coefficients: Vec<T>) -> Self {
        // For now we make it an error to input more coefficients than we can handle
        // In the future maybe we want to handle this more gracefully
        assert!(coefficients.len() <= N);
        coefficients.truncate(N);

        let degree = N.min(coefficients.len()) - 1;

        // Check for zero polynomial
        if degree == 0 && coefficients[0].eq(&T::zero()) {
            return Self::zero()
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

        let mut v = vec![];

        for i in 0..degree {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }

    /// Set a coefficient of the polynomial, recalculates the degree
    /// Ignores values beyond the dimension of the polynomial
    pub fn set_coeff(&mut self, index: usize, val: T) {
        if index < N && !val.is_zero() {

            let degree = match self.degree() {
                Some(d) if d < index => index,
                None => index,
                Some(d) => d,
            };

            let mut coefficients = self.coefficients.clone();


            // Safe unwrap since degree becomes index if it was None
            coefficients.resize(degree + 1, T::zero());
            coefficients[index] = val;

            self.degree = Some(degree);
            self.coefficients = coefficients;
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
    T: FiniteField + Clone + Default,
{
    fn default() -> Self {
        Self::init()
    }
}
