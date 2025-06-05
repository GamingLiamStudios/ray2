use std::ops::{
    Add,
    AddAssign,
    Div,
    Mul,
    Neg,
    Sub,
    SubAssign,
};

use crate::math::{
    One,
    SquareRoot,
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy + Clone> Vec3<T> {
    pub const fn new(
        x: T,
        y: T,
        z: T,
    ) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: T) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub const fn from_array(v: [T; 3]) -> Self {
        let [x, y, z] = v;
        Self::new(x, y, z)
    }

    pub const fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub const fn as_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub fn map(
        &self,
        f: impl Fn(T) -> T,
    ) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn dot(
        &self,
        rhs: &Self,
    ) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(
        lhs: &Self,
        rhs: &Self,
    ) -> Self
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
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
        *self / self.length()
    }

    pub fn lerp(
        lhs: &Self,
        rhs: &Self,
        t: T, // Horrid, but hey it works
    ) -> Self
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One,
    {
        lhs.map(|v| v * (T::ONE - t)) + rhs.map(|v| v * t)
    }
}

/// Scalar Operations
impl<T: Copy + Add<Output = T>> Add<T> for Vec3<T> {
    type Output = Self;

    fn add(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v + rhs)
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Vec3<T> {
    type Output = Self;

    fn sub(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v - rhs)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v * rhs)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(
        self,
        rhs: T,
    ) -> Self::Output {
        self.map(|v| v / rhs)
    }
}

/// Vector Operations
impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
