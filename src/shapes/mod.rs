mod solver;

use crate::traits::{Scalar, Vector, VectorHelper};

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
                    let r_sq = (*a_radius + *b_radius) * (*a_radius + *b_radius);
                    let min_t = solver::quadratic(
                        (b_delta - a_delta).length_sq(),
                        S::from(2) * (a_pos - b_pos).dot(&(b_delta - a_delta)),
                        (a_pos - b_pos).length_sq() - r_sq,
                    );

                    min_t
                        .map(|s| s.1)
                        .filter(|v| S::from(0) <= *v && v <= &S::from(1))
                        .is_some()
                }
                Shape::Cube { .. } => todo!(),
            },
            Shape::Cube { .. } => todo!(),
        }
    }
}
