use crate::polyvec::structures::{FiniteRing, RingModule};

use std::fmt::{self, Debug};

/// A `Matrix` is a collection of `Vector`s
pub struct Matrix<K, V>
where
    V: RingModule<K>,
    K: FiniteRing + Default,
{
    /// Internal representation as a list of elements of type `T`
    columns: Vec<V>,

    trace: K,

    /// Dimensions of the matrix
    dimensions: (usize, usize),
}

impl<K, V> Matrix<K, V>
where
    V: RingModule<K> + Clone,
    K: FiniteRing + Default,
{
    /// Initialise an empty `Matrix`
    ///      - `col_num`: number of columns
    ///      - `col_dim`: number of rows
    pub fn init_matrix(col_num: usize, col_dim: usize) -> Self {
        Self {
            columns: vec![V::init(col_dim); col_num],
            dimensions: (col_num, col_dim),
            trace: Default::default(),
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

    pub fn row(&self, index: usize) -> V {
        let n = self.dimensions().0;
        let mut t = V::init(n);

        for i in 0..n {
            t.set(i, self.columns[i].get(index));
        }

        t
    }

    /// Set a coefficient
    pub fn set(&mut self, _row: usize, _column: usize, _value: K) {
        unimplemented!()
    }

    pub fn vec_mul(&self, v: &V) -> V {
        assert!(v.dimension() == self.dimensions().0);

        let n = self.dimensions().0;

        let mut t: V = V::init(n);

        for j in 0..n {
            t.set(j, v.dot(&self.row(j)));
        }

        t
    }
}

impl<K, V> fmt::Debug for Matrix<K, V>
where
    V: RingModule<K> + Debug,
    K: FiniteRing + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}\n", self.columns)
    }
}
