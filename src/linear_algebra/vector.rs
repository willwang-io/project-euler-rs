use std::iter::Sum;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<T>(Vec<T>);

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    fn from(data: [T; N]) -> Self {
        Self(Vec::from(data))
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(data: Vec<T>) -> Self {
        Self(data)
    }
}

impl<T> Vector<T> {
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T> Mul<&Vector<T>> for &Vector<T>
where
    T: Copy + Mul<Output = T> + Sum<T>,
{
    type Output = T;

    fn mul(self, other: &Vector<T>) -> T {
        assert_eq!(self.size(), other.size());

        self.0.iter().zip(&other.0).map(|(&a, &b)| a * b).sum()
    }
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.size(), other.size());

        Self(
            self.0
                .into_iter()
                .zip(other.0)
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl<T: Copy + Add<Output = T>> Add<&Vector<T>> for &Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size(), other.size());

        Vector(self.0.iter().zip(&other.0).map(|(&a, &b)| a + b).collect())
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.size(), other.size());

        Self(
            self.0
                .into_iter()
                .zip(other.0)
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl<T: Copy + Sub<Output = T>> Sub<&Vector<T>> for &Vector<T> {
    type Output = Vector<T>;

    fn sub(self, other: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size(), other.size());

        Vector(self.0.iter().zip(&other.0).map(|(&a, &b)| a - b).collect())
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self(self.0.into_iter().map(|x| x * scalar).collect())
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for &Vector<T> {
    type Output = Vector<T>;

    fn mul(self, scalar: T) -> Vector<T> {
        Vector(self.0.iter().map(|&x| x * scalar).collect())
    }
}
