use std::{thread, time::Duration};

use macroquad::prelude::*;
use nous::{RigidBody, Shape, Simulation, Vec2};

#[macroquad::main("Simulation Test")]
async fn main() {
    let mut sim = Simulation::new();

    let a = RigidBody::new(
        Shape::Sphere { radius: 0.5 },
        1.,
        Vec2::ZERO,
        Vec2::new(-1., 0.),
        1.,
    );
    sim.spawn(a);

    let b = RigidBody::new(
        Shape::Sphere { radius: 0.5 },
        1.,
        Vec2::new(0.5, -2.),
        Vec2::new(0., 7.),
        1.,
    );
    sim.spawn(b);

    let scale = 40.;
    loop {
        clear_background(BLACK);

        for o in sim.objects() {
            match o.shape() {
                Shape::Sphere { radius } => draw_circle(
                    screen_width() / 2. + o.pos().x * scale,
                    screen_height() / 2. + o.pos().y * scale,
                    radius * scale,
                    WHITE,
                ),
                Shape::Cube { .. } => todo!(),
            }
        }
        let sleep = 1. / 60.;
        sim.tick(sleep / 5.);
        thread::sleep(Duration::from_secs_f32(sleep));

        // dbg!(sim.total_momentum());

        next_frame().await
    }
}
