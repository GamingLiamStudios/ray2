use std::ops::{
    Add,
    Div,
    Mul,
    Neg,
    Sub,
};

use crate::math::SquareRoot;

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T: Copy + Clone> Vec2<T> {
    pub const fn new(
        x: T,
        y: T,
    ) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: T) -> Self {
        Self { x: v, y: v }
    }

    pub const fn from_array(v: [T; 2]) -> Self {
        let [x, y] = v;
        Self::new(x, y)
    }

    pub const fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    pub const fn as_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn map(
        &self,
        f: impl Fn(T) -> T,
    ) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
        }
    }

    pub fn dot(
        &self,
        rhs: &Self,
    ) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length_squared(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.dot(self)
    }

    pub fn length(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + SquareRoot,
    {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self
    where
        T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + SquareRoot,
    {
        *self / self.length_squared()
    }
}

/// Scalar Operations
impl<T: Copy + Add<Output = T>> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v + rhs)
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v - rhs)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v * rhs)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v / rhs)
    }
}

/// Vector Operations
impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
