use itertools::Itertools;

use crate::{
    shapes::Shape,
    traits::{Scalar, Vector},
};

pub struct Simulation<V: Vector<S>, S> {
    objects: Vec<RigidBody<V, S>>,
}

impl<V: Vector<S>, S: Scalar> Simulation<V, S> {
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

    pub fn tick(&mut self, delta: S) {
        for (a, b) in (0..self.objects.len()).tuple_combinations() {
            if a != b {
                debug_assert!(a < b);
                let (first, last) = self.objects.split_at_mut(b);
                let a = &mut first[a];
                let b = &mut last[0];
                a.intersect_tick(b, delta);
            }
        }

        for body in &mut self.objects {
            body.move_tick(delta);
        }
    }
}

impl<V: Vector<S>, S: Scalar> Default for Simulation<V, S> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RigidBody<V: Vector<S>, S> {
    shape: Shape<V, S>,
    weight: S,
    pos: V,
    vel: V,
    collision: Option<()>,
}

impl<V: Vector<S>, S: Scalar> RigidBody<V, S> {
    pub fn new(shape: Shape<V, S>, weight: S, pos: V, vel: V) -> Self {
        Self {
            shape,
            weight,
            pos,
            vel,
            collision: None,
        }
    }

    pub fn intersect_tick(&mut self, other: &mut RigidBody<V, S>, delta: S) {
        let intersection = Shape::intersect(
            &self.shape,
            self.pos,
            self.vel * delta,
            &other.shape,
            other.pos,
            other.vel * delta,
        );
        if intersection {
            self.collision = Some(());
            other.collision = Some(());
        }
    }

    pub fn move_tick(&mut self, delta: S) {
        if self.collision.is_none() {
            self.pos = self.pos + self.vel * delta;
        }
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
