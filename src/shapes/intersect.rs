use crate::traits::{Scalar, Vector};

use super::{solver, BodyProperties, Intersection};

pub fn sphere_sphere<V: Vector<S>, S: Scalar>(
    a_radius: S,
    b_radius: S,
    mut a: BodyProperties<V, S>,
    mut b: BodyProperties<V, S>,
) -> Option<Intersection<V, S>> {
    let a_pos = a.pos;
    let a_delta = a.vel;
    let b_pos = b.pos;
    let b_delta = b.vel;

    // Solve this equation with minimal t
    // ((a_pos + a_delta * t) - (b_pos + b_delta * t)).length_sq() == (a_radius + b_radius) * (a_radius + b_radius)
    let r_sq = (a_radius + b_radius) * (a_radius + b_radius);
    let min_t = solver::quadratic(
        (b_delta - a_delta).length_sq(),
        S::from(2) * (a_pos - b_pos).dot(&(b_delta - a_delta)),
        (a_pos - b_pos).length_sq() - r_sq,
    );

    min_t
        .map(|s| s.1)
        .filter(|v| S::from(0) <= *v && v <= &S::from(1))
        .map(|t| {
            a.pos = a_pos + a_delta * t;
            b.pos = b_pos + b_delta * t;
            Intersection {
                t,
                other_normal: (a.pos - b.pos).normalized(),
                self_properties: a,
                other_properties: b,
            }
        })
}
