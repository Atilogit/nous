use std::ops::{Add, Mul, Sub};

pub trait Vector<S>:
    Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<S, Output = Self> + Sized + Copy
{
    fn length_sq(&self) -> S;
}

pub trait Scalar: Copy {}

impl Vector<f32> for glam::Vec2 {
    fn length_sq(&self) -> f32 {
        self.length_squared()
    }
}

impl Scalar for f32 {}
