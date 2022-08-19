mod intersect;
mod solver;

use crate::traits::{Scalar, Vector};

#[derive(Clone, Debug)]
pub struct BodyProperties<V, S> {
    pub shape: Shape<V, S>,
    pub mass: S,
    pub pos: V,
    pub restitution: S,
    pub bounding_radius: S,
    scaled_vel_length: S,
    actual_vel_length: S,
    scaled_vel: V,
    actual_vel: V,
}

impl<V: Vector<S>, S: Scalar> BodyProperties<V, S> {
    pub fn new(shape: Shape<V, S>, mass: S, pos: V, vel: V, restitution: S) -> Self {
        let vel_len = vel.length();
        Self {
            bounding_radius: shape.bounding_radius(),
            shape,
            mass,
            pos,
            restitution,
            scaled_vel_length: vel_len,
            actual_vel_length: vel_len,
            scaled_vel: vel,
            actual_vel: vel,
        }
    }

    pub fn apply_delta(&self, delta: S) -> Self {
        Self {
            shape: self.shape.clone(),
            mass: self.mass,
            pos: self.pos,
            restitution: self.restitution,
            bounding_radius: self.bounding_radius,
            scaled_vel: self.actual_vel * delta,
            actual_vel: self.actual_vel,
            scaled_vel_length: self.actual_vel_length * delta,
            actual_vel_length: self.actual_vel_length,
        }
    }

    pub fn undo_delta(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            mass: self.mass,
            pos: self.pos,
            restitution: self.restitution,
            bounding_radius: self.bounding_radius,
            scaled_vel: self.actual_vel,
            actual_vel: self.actual_vel,
            scaled_vel_length: self.actual_vel_length,
            actual_vel_length: self.actual_vel_length,
        }
    }

    pub fn vel(&self) -> V {
        self.scaled_vel
    }

    pub fn vel_len(&self) -> S {
        self.scaled_vel_length
    }

    pub fn set_vel(&mut self, vel: V) {
        let vel_len = vel.length();
        self.actual_vel = vel;
        self.scaled_vel = vel;
        self.scaled_vel_length = vel_len;
        self.actual_vel_length = vel_len;
    }
}

#[derive(Clone, Debug)]
pub enum Shape<V, S> {
    Sphere { radius: S },
    Cube { scale: V },
}

#[derive(Debug)]
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
    pub fn bounding_radius(&self) -> S {
        match self {
            Shape::Sphere { radius } => *radius,
            Shape::Cube { .. } => todo!(),
        }
    }

    pub fn intersect(
        a: BodyProperties<V, S>,
        b: BodyProperties<V, S>,
    ) -> Option<Intersection<V, S>> {
        let distance_limit = a.vel_len() + a.bounding_radius + b.bounding_radius + b.vel_len();
        if (a.pos - b.pos).length_sq() > distance_limit * distance_limit {
            // Skip if bodies are too far apart to collide in this tick
            None
        } else {
            match a.shape {
                Shape::Sphere { .. } => match b.shape {
                    Shape::Sphere { .. } => intersect::sphere_sphere(a, b),
                    Shape::Cube { .. } => todo!(),
                },
                Shape::Cube { .. } => todo!(),
            }
        }
    }

    pub fn collide(i: &Intersection<V, S>, self_properties: &mut BodyProperties<V, S>, delta: S) {
        // Do subtick until collision
        self_properties.pos = self_properties.pos + self_properties.vel() * i.t * delta;

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
        let vn = dv.dot(i.other_normal);

        // Calculate impulse scalar
        let j = (-(S::from(1) + cr) * vn) / (S::from(1) / ma + S::from(1) / mb);

        // Apply impulse
        let impulse = i.other_normal * j;
        self_properties.set_vel(self_properties.vel() - impulse * (S::from(1) / ma));

        // Remaining subtick with new velocity
        self_properties.pos =
            self_properties.pos + self_properties.vel() * (S::from(1) - i.t) * delta;
    }
}
