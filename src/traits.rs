use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait Vector<S: Scalar>:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<S, Output = Self>
    + Div<S, Output = Self>
    + Sized
    + Copy
    + Debug
{
    fn dot(&self, other: &Self) -> S;

    fn length_sq(&self) -> S {
        self.dot(self)
    }

    fn length(&self) -> S {
        self.length_sq().sqrt()
    }

    fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * S::from(2) * self.dot(normal)
    }

    fn normalized(&self) -> Self {
        let sq = self.length_sq();
        if sq == S::from(0) {
            *self
        } else {
            *self / sq.sqrt()
        }
    }
}

pub trait Scalar:
    Copy
    + Neg<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Add<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + From<i16>
    + Debug
    + Sum
{
    fn sqrt(&self) -> Self;
    fn min(&self, other: &Self) -> Self {
        if self < other {
            *self
        } else {
            *other
        }
    }
}

impl Vector<f32> for glam::Vec2 {
    fn dot(&self, other: &Self) -> f32 {
        glam::Vec2::dot(*self, *other)
    }
}

impl Scalar for f32 {
    fn sqrt(&self) -> Self {
        // Somehow errors with clippy but compiles fine
        f32::sqrt(*self)
    }
}
