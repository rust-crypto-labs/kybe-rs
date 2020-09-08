// Finite field element
pub trait FiniteField: Sized + FiniteRing {
    /// Returns the multiplicative inverse of the element
    fn inv(&self) -> Result<Self, String>;

    /// Defines the divison of two elements
    fn div(&self, other: &Self) -> Result<Self, String>;
}

// Finite Ring element
pub trait FiniteRing: Sized + Eq {
    /// Check if the element is the additive identity of the field
    fn is_zero(&self) -> bool;

    /// Returns the dimension of the finite field
    fn dimension(&self) -> usize;

    /// Returns the additive identity of the field
    fn zero(&self) -> Self;

    /// Returns the multiplicative identity of the field
    fn one(&self) -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;

    /// Defines the multiplication of two elements
    fn mul(&self, other: &Self) -> Self;

    /// Converts the element to a bytes representation
    fn into_bytes(self) -> Vec<u8>;

    /// Converts a bytes representation to an element of the finite field
    fn from_bytes(bytes: &[u8]) -> Result<Self, String>;
}
