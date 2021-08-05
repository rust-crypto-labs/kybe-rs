//! Matrix
//!
//! Matrix definiton to match polyvec

use crate::polyvec::polyvec::PolyVec;
use crate::polyvec::structures::{FiniteRing, RingModule};

use std::fmt::{self, Debug};

/// A `Matrix` is a collection of `Vector`s
pub struct Matrix<K, const X: usize, const Y: usize>
where
    K: FiniteRing + Clone + Default,
{
    /// Internal representation as a list of elements of type `T`
    coefficients: Vec<K>,

    /// Dimensions of the matrix
    dimensions: (usize, usize),
}

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
where
    K: FiniteRing + Clone + Default,
{
    /// Initialise an empty `Matrix`
    ///      - `col_num`: number of columns
    ///      - `col_dim`: number of rows
    pub fn init_matrix(col_num: usize, col_dim: usize) -> Self {
        Self {
            coefficients: vec![Default::default(); col_num * col_dim],
            dimensions: (col_num, col_dim),
        }
    }

    /// Return the matrix dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// Return a specific row
    pub fn row(&self, index: usize) -> PolyVec<K, X> {
        let (cols, rows) = self.dimensions();
        let mut t = PolyVec::<K,X>::init();

        for i in 0..cols {
            t.set(i, self.coefficients[index * rows + i].clone());
        }

        t
    }

    /// Return a specific column
    pub fn column(&self, index: usize) -> PolyVec<K,Y> {
        let (cols, rows) = self.dimensions();
        let mut t = PolyVec::<K,Y>::init();

        for i in 0..rows {
            t.set(i, self.coefficients[index * i + cols].clone());
        }

        t
    }

    /// Set a coefficient
    pub fn set(&mut self, row: usize, column: usize, value: K) {
        let (cols, rows) = self.dimensions();
        assert!((column < cols) && (row < rows));

        self.coefficients[row * rows + column] = value;
    }

    /// Get a coefficient
    pub fn get(&self, row: usize, column: usize) -> K {
        let (cols, rows) = self.dimensions();
        assert!((column < cols) && (row < rows));

        self.coefficients[row * rows + column].clone()
    }

    /// Perform a matrix vector multiplication
    pub fn vec_mul(&self, v: &PolyVec<K,X>) -> PolyVec<K,Y> {
        let (cols, rows) = self.dimensions();
        assert!(v.dimension() == rows);

        let mut t = PolyVec::<K,Y>::init();

        for j in 0..cols {
            t.set(j, v.dot(&self.row(j)));
        }

        t
    }
}

impl<K, const X: usize, const Y: usize> fmt::Debug for Matrix<K,X,Y>
where
    K: FiniteRing + Clone + Default + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}\n", self.coefficients)
    }
}
