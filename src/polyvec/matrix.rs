use crate::polyvec::vector::Vector;

use std::{
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};

/// A `Matrix` is a collection of `Vector`s
pub struct Matrix<T: Vector> {
    /// Internal representation as a list of elements of type `T`
    columns: Vec<T>,

    /// Dimensions of the matrix
    dimensions: (usize, usize),
}

impl<T: Vector> Matrix<T>
where
    T: Clone,
{
    /// Initialise an empty `Matrix`
    ///      - `col_num`: number of columns
    ///      - `col_dim`: number of rows
    pub fn init(col_num: usize, col_dim: usize) -> Self {
        Self {
            columns: vec![T::init(col_dim); col_num],
            dimensions: (col_num, col_dim),
        }
    }

    /// Return the matrix dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// Swap two columns of the matrix
    pub fn swap(&mut self, i: usize, j: usize) {
        self.columns.swap(i, j);
    }

    pub fn vec_mul(&self, _v: &T) -> &T {
        unimplemented!();
    }
}

/// Direct access to a column
impl<T> Index<usize> for Matrix<T>
where
    T: Vector,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.columns[index]
    }
}

/// Direct access to a column (mutable)
impl<T> IndexMut<usize> for Matrix<T>
where
    T: Vector,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.columns[index]
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: Vector + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}\n", self.columns)
    }
}
