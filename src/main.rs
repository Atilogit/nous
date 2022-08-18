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
        Vec2::new(0., 8.),
        1.,
    );
    sim.spawn(b);

    let scale = 40.;
    let delta_step = 0.5;
    let draw_vel = false;
    loop {
        clear_background(BLACK);

        for o in sim.objects() {
            match o.shape() {
                Shape::Sphere { radius } => {
                    draw_circle_lines(
                        screen_width() / 2. + o.pos().x * scale,
                        screen_height() / 2. + o.pos().y * scale,
                        radius * scale,
                        1.,
                        WHITE,
                    );
                    if draw_vel {
                        draw_line(
                            screen_width() / 2. + o.pos().x * scale,
                            screen_height() / 2. + o.pos().y * scale,
                            screen_width() / 2. + (o.pos().x + o.vel().x * delta_step) * scale,
                            screen_height() / 2. + (o.pos().y + o.vel().y * delta_step) * scale,
                            1.,
                            RED,
                        )
                    }
                }
                Shape::Cube { .. } => todo!(),
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            sim.tick(delta_step);
        }
        // sim.tick(0.001);

        next_frame().await
    }
}
