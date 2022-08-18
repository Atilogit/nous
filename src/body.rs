use crate::{
    shapes::{BodyProperties, Intersection, Shape},
    traits::{Scalar, Vector},
};

pub struct RigidBody<V, S> {
    properties: BodyProperties<V, S>,
    intersection: Option<Intersection<V, S>>,
}

impl<V: Vector<S>, S: Scalar> RigidBody<V, S> {
    pub fn new(shape: Shape<V, S>, weight: S, pos: V, vel: V) -> Self {
        Self {
            properties: BodyProperties::new(shape, weight, pos, vel),
            intersection: None,
        }
    }

    pub fn intersect_tick(&mut self, other: &mut RigidBody<V, S>, delta: S) {
        let intersection = Shape::intersect(
            self.properties.apply_delta(delta),
            other.properties.apply_delta(delta),
        );
        // TODO Only take earliest collision
        other.intersection = intersection.as_ref().map(Intersection::invert);
        self.intersection = intersection;
    }

    pub fn move_tick(&mut self, delta: S) {
        if let Some(intersection) = &self.intersection {
            Shape::collide(intersection, &mut self.properties)
        } else {
            self.properties.pos = self.properties.pos + self.properties.vel * delta;
        }
        self.intersection = None;
    }

    pub fn shape(&self) -> &Shape<V, S> {
        &self.properties.shape
    }

    pub fn weight(&self) -> &S {
        &self.properties.weight
    }

    pub fn pos(&self) -> V {
        self.properties.pos
    }

    pub fn vel(&self) -> V {
        self.properties.vel
    }
}
