use crate::polyvec::structures::{FiniteField, FiniteRing};
use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

/// Represents a polynomial in the ring T[X]/(X^n + 1)
#[derive(Clone)]
pub struct Polynomial<T>
where
    T: FiniteField + Default,
{
    /// Coefficients of the polynomial
    pub coefficients: Vec<T>,

    /// Degree of the polynomial (the zero polynomial has degree < 0)
    pub degree: i32,

    /// Dimension of the ring as as a vector space over T
    pub n: usize,
}

impl<T> FiniteRing for Polynomial<T>
where
    T: FiniteField + Clone + Default,
{
    fn is_zero(&self) -> bool {
        self.degree() < 0
    }

    fn zero(&self) -> Self {
        let t: T = Default::default();
        Polynomial {
            coefficients: vec![t.zero()],
            degree: -1,
            n: self.n,
        }
    }

    fn one(&self) -> Self {
        let t: T = Default::default();
        Polynomial {
            coefficients: vec![t.one()],
            degree: 0,
            n: self.n,
        }
    }

    fn neg(&self) -> Self {
        // If the polynomial is already zero, do nothing
        if self.is_zero() {
            return self.clone();
        }

        // Otherwise the degree is positive
        let degree = self.degree();
        let t: T = Default::default();
        let mut coefficients = vec![t.zero(); degree.try_into().unwrap()];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = c.neg();
        }
        Polynomial {
            coefficients,
            degree,
            n: self.n,
        }
    }

    fn add(&self, other: &Self) -> Self {
        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        let mut degree = self.degree().max(other.degree);
        let t: T = Default::default();
        let mut coefficients = vec![t.zero(); degree.try_into().unwrap()];
        for (i, c) in self.coefficients.iter().enumerate() {
            coefficients[i] = other.coefficients[i].add(c);
        }

        // Diminish degree if leading coefficient is zero
        let mut leading: usize = degree.try_into().unwrap();
        leading = leading - 1;
        while degree > 0 && coefficients[leading].eq(&t.zero()) {
            degree -= 1;
            leading -= 1;
        }

        // Check whether the result is zero
        if leading == 0 && coefficients[0].eq(&t.zero()) {
            degree = -1;
        }

        Polynomial {
            coefficients,
            degree,
            n: self.n,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// MOD X^N + 1  !
    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() {
            return self.clone();
        }
        if other.is_zero() {
            return other.clone();
        }

        let t: T = Default::default();
        let coeffs = vec![t.zero(); self.n];

        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                let c = a.mul(&b);
                let k = i + j;
                if k < self.n {
                    coeffs[k].add(&c);
                } else {
                    // X^n = -1
                    coeffs[k % self.n].sub(&c);
                }
            }
        }

        // Reduce degree if appropriate
        let mut leading = self.n - 1;
        while leading > 0 && coeffs[leading].eq(&t.zero()) {
            leading -= 1;
        }

        // Check for null polynomial (shouldn't happen but still)
        if leading == 0 && coeffs[0].eq(&t.zero()) {
            return self.zero();
        }

        Self {
            coefficients: coeffs,
            degree: leading.try_into().unwrap(),
            n: self.n,
        }
    }
}

impl<T> PartialEq for Polynomial<T>
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

impl<T> Eq for Polynomial<T> where T: FiniteField + Default {}

impl<T> Polynomial<T>
where
    T: FiniteField + Clone + Default,
{
    pub fn init(n: usize) -> Self {
        Self::from_vec(vec![Default::default()], n)
    }

    pub fn dimension(&self) -> usize {
        self.n
    }

    pub fn from_vec(coefficients: Vec<T>, n: usize) -> Self {
        // For now we make it an error to input more coefficients than we can handle
        // In the future maybe we want to handle this more gracefully
        assert!(coefficients.len() <= n);

        let mut degree = n.min(coefficients.len()).try_into().unwrap();

        let t: T = Default::default();
        // Check for zero polynomial
        if degree == 0 && coefficients[0].eq(&t.zero()) {
            degree = -1;
        }

        Polynomial {
            coefficients,
            degree,
            n,
        }
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn mulf(&self, other: &T) -> Self {
        let mut v = vec![];

        for i in 0..self.degree().try_into().unwrap() {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v, self.n)
    }
}

impl<T> Index<usize> for Polynomial<T>
where
    T: FiniteField + Default,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.coefficients[index]
    }
}

impl<T> IndexMut<usize> for Polynomial<T>
where
    T: FiniteField + Default,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.coefficients[index]
    }
}

impl<T> Default for Polynomial<T>
where
    T: FiniteField + Clone + Default,
{
    fn default() -> Self {
        Self::init(1)
    }
}
