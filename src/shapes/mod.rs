mod intersect;
mod solver;

use crate::traits::{Scalar, Vector};

#[derive(Clone)]
pub struct BodyProperties<V, S> {
    pub shape: Shape<V, S>,
    pub mass: S,
    pub pos: V,
    pub restitution: S,
    vel: V,
    actual_vel: V,
}

impl<V: Vector<S>, S: Scalar> BodyProperties<V, S> {
    pub fn new(shape: Shape<V, S>, mass: S, pos: V, vel: V, restitution: S) -> Self {
        Self {
            shape,
            mass,
            pos,
            restitution,
            vel,
            actual_vel: vel,
        }
    }

    pub fn apply_delta(&self, delta: S) -> Self {
        Self {
            shape: self.shape.clone(),
            mass: self.mass,
            pos: self.pos,
            restitution: self.restitution,
            vel: self.vel * delta,
            actual_vel: self.vel,
        }
    }

    pub fn undo_delta(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            mass: self.mass,
            pos: self.pos,
            restitution: self.restitution,
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
            t: self.t,
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
        let cr = self_properties
            .restitution
            .min(&i.other_properties.restitution);
        let ma = self_properties.mass;
        let mb = i.other_properties.mass;
        let ua = self_properties.vel();
        let ub = i.other_properties.undo_delta().vel();

        // Calculate relative velocity
        let dv = ub - ua;
        // Calculate relative velocity in terms of the normal direction
        let vn = dv.dot(&i.other_normal);

        // Do not resolve if velocities are separating
        if vn > S::from(0) {
            // Calculate impulse scalar
            let j = (-(S::from(1) + cr) * vn) / (S::from(1) / ma + S::from(1) / mb);

            // Apply impulse
            let impulse = i.other_normal * j;
            self_properties.set_vel(self_properties.vel() - impulse * (S::from(1) / ma));
        }

        dbg!(self_properties.vel().length());
    }
}
