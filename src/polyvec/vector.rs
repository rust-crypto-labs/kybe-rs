/// The `Vector` trait describes the general properties of an element in a vector space.
pub trait Vector {
    /// Returns the vector's dimension
    fn dimension(&self) -> usize;

    /// Add two vectors together
    fn add(&self, other: &Self) -> Self;

    /// Substract two vectors
    fn sub(&self, other: &Self) -> Self;

    /// Initialise vector type
    fn init(dimension: usize) -> Self;

    /// Basis vector
    fn basis_vector(&self, position: usize) -> Self;
}

/// The `Dot` trait allows the computation of dot products with values in `T`
pub trait Dot<T> {
    fn dot(&self, other: &Self) -> T;
}
