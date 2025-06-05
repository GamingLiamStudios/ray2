mod vec2;
mod vec3;

pub use vec2::Vec2;
pub use vec3::Vec3;

pub trait SquareRoot: Copy {
    fn sqrt(self) -> Self;
}

impl SquareRoot for f32 {
    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
}

impl SquareRoot for f64 {
    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
}

pub trait One: Copy {
    const ONE: Self;
}

impl One for f64 {
    const ONE: Self = 1.0;
}
