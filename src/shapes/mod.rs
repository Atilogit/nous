mod intersect;
mod solver;

use crate::traits::{Scalar, Vector};

#[derive(Clone)]
pub struct BodyProperties<V, S> {
    pub shape: Shape<V, S>,
    pub weight: S,
    pub pos: V,
    vel: V,
    actual_vel: V,
}

impl<V: Vector<S>, S: Scalar> BodyProperties<V, S> {
    pub fn new(shape: Shape<V, S>, weight: S, pos: V, vel: V) -> Self {
        Self {
            shape,
            weight,
            pos,
            vel,
            actual_vel: vel,
        }
    }

    pub fn apply_delta(&self, delta: S) -> Self {
        Self {
            shape: self.shape.clone(),
            weight: self.weight,
            pos: self.pos,
            vel: self.vel * delta,
            actual_vel: self.vel,
        }
    }

    pub fn undo_delta(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            weight: self.weight,
            pos: self.pos,
            vel: self.actual_vel,
            actual_vel: self.actual_vel,
        }
    }

    pub fn vel(&self) -> V {
        self.vel
    }

    pub fn set_vel(&mut self, vel: V) {
        self.vel = vel;
        self.actual_vel = vel;
    }
}

#[derive(Clone)]
pub enum Shape<V, S> {
    Sphere { radius: S },
    Cube { scale: V },
}

pub struct Intersection<V, S> {
    pub t: S,
    pub other_normal: V,
    pub self_properties: BodyProperties<V, S>,
    pub other_properties: BodyProperties<V, S>,
}

impl<V: Vector<S>, S: Scalar> Intersection<V, S> {
    pub fn invert(&self) -> Self {
        Self {
            t: self.t.clone(),
            self_properties: self.other_properties.clone(),
            other_properties: self.self_properties.clone(),
            other_normal: self.other_normal * S::from(-1),
        }
    }
}

impl<V: Vector<S>, S: Scalar> Shape<V, S> {
    pub fn intersect(
        a: BodyProperties<V, S>,
        b: BodyProperties<V, S>,
    ) -> Option<Intersection<V, S>> {
        match a.shape {
            Shape::Sphere { radius: a_radius } => match b.shape {
                Shape::Sphere { radius: b_radius } => {
                    intersect::sphere_sphere(a_radius, b_radius, a, b)
                }
                Shape::Cube { .. } => todo!(),
            },
            Shape::Cube { .. } => todo!(),
        }
    }

    pub fn collide(i: &Intersection<V, S>, self_properties: &mut BodyProperties<V, S>) {
        let cr = S::from(0);
        let ma = self_properties.weight;
        let mb = i.other_properties.weight;
        let ua = self_properties.vel().length();
        let ub = i.other_properties.undo_delta().vel().length();

        let new_dir = self_properties.vel.reflect(&i.other_normal).normalized();
        self_properties.set_vel(new_dir * ((cr * mb * (ub - ua) + ma * ua + mb * ub) / (ma + mb)));

        // let j = ((ma * mb) / (ma + mb)) * (S::from(1) + cr) * (i.other_normal.dot(&(ub - ua)));
        // self_properties.set_vel(self_properties.vel() + i.other_normal * (j / ma));

        dbg!(self_properties.vel().length());
    }
}
