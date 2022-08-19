use crate::{
    shapes::{BodyProperties, Intersection, Shape},
    traits::{Scalar, Vector},
};

pub struct RigidBody<V, S> {
    properties: BodyProperties<V, S>,
    intersection: Option<Intersection<V, S>>,
}

impl<V: Vector<S>, S: Scalar> RigidBody<V, S> {
    pub fn new(shape: Shape<V, S>, mass: S, pos: V, vel: V, restitution: S) -> Self {
        Self {
            properties: BodyProperties::new(shape, mass, pos, vel, restitution),
            intersection: None,
        }
    }

    pub fn intersect_tick(&mut self, other: &mut RigidBody<V, S>, delta: S) {
        let intersection = Shape::intersect(
            self.properties.apply_delta(delta),
            other.properties.apply_delta(delta),
        );
        other.set_intersection(intersection.as_ref().map(Intersection::invert));
        self.set_intersection(intersection);
    }

    fn set_intersection(&mut self, i: Option<Intersection<V, S>>) {
        if let Some(i) = i {
            if let Some(current) = &self.intersection {
                if i.t < current.t {
                    self.intersection = Some(i);
                }
            } else {
                self.intersection = Some(i);
            }
        }
    }

    pub fn move_tick(&mut self, delta: S) {
        if let Some(intersection) = &self.intersection {
            Shape::collide(intersection, &mut self.properties)
        } else {
            self.properties.pos = self.properties.pos + self.properties.vel() * delta;
        }
        // Reset intersections
        self.intersection = None;
    }

    pub fn shape(&self) -> &Shape<V, S> {
        &self.properties.shape
    }

    pub fn mass(&self) -> S {
        self.properties.mass
    }

    pub fn pos(&self) -> V {
        self.properties.pos
    }

    pub fn vel(&self) -> V {
        self.properties.vel()
    }
}
