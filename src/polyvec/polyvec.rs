use crate::polyvec::structures::{FiniteRing, RingModule};

#[derive(Clone)]
pub struct PolyVec<T: FiniteRing> {
    pub coefficients: Vec<T>,
    pub dimension: usize,
}

impl<T> RingModule<T> for PolyVec<T>
where
    T: FiniteRing + Clone + Default,
{
    fn get(&self, position: usize) -> T {
        self.coefficients[position].clone()
    }

    fn set(&mut self, position: usize, value: T) {
        self.coefficients[position] = value;
    }

    fn zero(&self) -> Self {
        Self::init(self.dimension())
    }

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

    fn is_zero(&self) -> bool {
        if self.dimension == 0 {
            true
        } else {
            self.coefficients.iter().all(|c| c.is_zero())
        }
    }

    fn neg(&self) -> Self {
        let t = Self::init(self.dimension());
        t.sub(self)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.dimension(), other.dimension());
        let mut v = vec![Default::default(); self.dimension()];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].add(&other.coefficients[i]);
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

    fn dot(&self, other: &Self) -> T {
        assert_eq!(self.dimension(), other.dimension());
        let t: T = Default::default();
        let mut v = t.zero();

        for i in 0..self.dimension() {
            v = v.add(&self.coefficients[i].mul(&other.coefficients[i]))
        }
        v
    }

    fn mulf(&self, other: &T) -> Self {
        let mut v = vec![];

        for i in 0..self.dimension() {
            v[i] = self.coefficients[i].mul(other)
        }
        Self::from_vec(v)
    }
}

impl<T> Default for PolyVec<T>
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

impl<T> PolyVec<T>
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
}
