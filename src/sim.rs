use itertools::Itertools;

use crate::{
    body::RigidBody,
    traits::{Scalar, Vector},
};

pub struct Simulation<V: Vector<S>, S: Scalar> {
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
        #[cfg(debug_assertions)]
        let total_momentum = self.total_momentum();

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

        #[cfg(debug_assertions)]
        debug_assert!((self.total_momentum() - total_momentum) < S::from(1) / S::from(1000))
    }

    pub fn total_momentum(&self) -> S {
        self.objects().map(|o| o.weight() * o.vel().length()).sum()
    }
}

impl<V: Vector<S>, S: Scalar> Default for Simulation<V, S> {
    fn default() -> Self {
        Self::new()
    }
}
