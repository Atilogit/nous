use crate::{
    shapes::Shape,
    traits::{Scalar, Vector},
};

pub struct Simulation<V: Vector<S>, S> {
    objects: Vec<RigidBody<V, S>>,
}

impl<V: Vector<S>, S> Simulation<V, S> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn spawn(&mut self, body: RigidBody<V, S>) {
        self.objects.push(body)
    }

    pub fn objects(&self) -> impl Iterator<Item = &RigidBody<V, S>> + '_ {
        self.objects.iter()
    }

    pub fn tick() {}
}

impl<V: Vector<S>, S> Default for Simulation<V, S> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RigidBody<V: Vector<S>, S> {
    shape: Shape<V, S>,
    weight: S,
    pos: V,
    vel: V,
}

impl<V: Vector<S>, S: Scalar> RigidBody<V, S> {
    pub fn new(shape: Shape<V, S>, weight: S, pos: V, vel: V) -> Self {
        Self {
            shape,
            weight,
            pos,
            vel,
        }
    }

    pub fn intersect_tick(&self, other: &RigidBody<V, S>) -> bool {
        Shape::intersect(
            &self.shape,
            self.pos,
            self.vel,
            &other.shape,
            other.pos,
            other.vel,
        )
    }

    pub fn shape(&self) -> &Shape<V, S> {
        &self.shape
    }

    pub fn weight(&self) -> &S {
        &self.weight
    }

    pub fn pos(&self) -> V {
        self.pos
    }

    pub fn vel(&self) -> V {
        self.vel
    }
}
