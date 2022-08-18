use crate::traits::{Scalar, Vector};

pub enum Shape<V, S> {
    Sphere { radius: S },
    Cube { scale: V },
}

impl<V: Vector<S>, S: Scalar> Shape<V, S> {
    pub fn intersect(
        a: &Shape<V, S>,
        a_pos: V,
        a_delta: V,
        b: &Shape<V, S>,
        b_pos: V,
        b_delta: V,
    ) -> bool {
        match a {
            Shape::Sphere { radius: a_radius } => match b {
                Shape::Sphere { radius: b_radius } => {
                    // Solve this equation with minimal t
                    // ((a_pos + a_delta * t) - (b_pos + b_delta * t)).length_sq() == (a_radius + b_radius) * (a_radius + b_radius)
                    true
                }
                Shape::Cube { .. } => todo!(),
            },
            Shape::Cube { .. } => todo!(),
        }
    }
}
