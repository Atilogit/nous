mod solver;

use crate::traits::{Scalar, Vector, VectorHelper};

#[derive(Clone)]
pub enum Shape<V, S> {
    Sphere { radius: S },
    Cube { scale: V },
}

pub struct Intersection<V, S> {
    pub t: S,
    pub self_pos: V,
    pub other_pos: V,
    pub self_shape: Shape<V, S>,
    pub other_shape: Shape<V, S>,
}

impl<V: Clone, S: Clone> Intersection<V, S> {
    pub fn invert(&self) -> Self {
        Self {
            t: self.t.clone(),
            self_pos: self.other_pos.clone(),
            other_pos: self.self_pos.clone(),
            self_shape: self.other_shape.clone(),
            other_shape: self.self_shape.clone(),
        }
    }
}

impl<V: Vector<S>, S: Scalar> Shape<V, S> {
    pub fn intersect(
        a: &Shape<V, S>,
        a_pos: V,
        a_delta: V,
        b: &Shape<V, S>,
        b_pos: V,
        b_delta: V,
    ) -> Option<Intersection<V, S>> {
        match a {
            Shape::Sphere { radius: a_radius } => match b {
                Shape::Sphere { radius: b_radius } => {
                    intersect_sphere_sphere(a_radius, a_pos, a_delta, b_radius, b_pos, b_delta)
                }
                Shape::Cube { .. } => todo!(),
            },
            Shape::Cube { .. } => todo!(),
        }
    }

    pub fn collide(intersection: &Intersection<V, S>) {
        match intersection.self_shape {
            Shape::Sphere { radius: self_rad } => match intersection.other_shape {
                Shape::Sphere { radius: other_rad } => {
                    collide_sphere_sphere(intersection, self_rad, other_rad)
                }
                Shape::Cube { .. } => todo!(),
            },
            Shape::Cube { .. } => todo!(),
        }
    }
}

fn collide_sphere_sphere<V: Vector<S>, S: Scalar>(
    intersection: &Intersection<V, S>,
    self_rad: S,
    other_rad: S,
) {
}

fn intersect_sphere_sphere<V: Vector<S>, S: Scalar>(
    a_radius: &S,
    a_pos: V,
    a_delta: V,
    b_radius: &S,
    b_pos: V,
    b_delta: V,
) -> Option<Intersection<V, S>> {
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
        .map(|t| Intersection {
            t,
            self_pos: a_pos + a_delta * t,
            other_pos: b_pos + b_delta * t,
            self_shape: Shape::Sphere { radius: *a_radius },
            other_shape: Shape::Sphere { radius: *b_radius },
        })
}
