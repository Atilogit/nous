use macroquad::prelude::*;
use nous::{RigidBody, Shape, Simulation, Vec2};

#[macroquad::main("Simulation Test")]
async fn main() {
    let mut sim = Simulation::new();

    sim.spawn(RigidBody::new(
        Shape::Sphere { radius: 0.5 },
        1.,
        Vec2::ZERO,
        Vec2::new(-1., 0.),
        1.,
    ));
    sim.spawn(RigidBody::new(
        Shape::Sphere { radius: 0.5 },
        1.,
        Vec2::new(0.5, -2.),
        Vec2::new(0., 8.),
        1.,
    ));
    sim.spawn(RigidBody::new(
        Shape::Sphere { radius: 0.5 },
        1.,
        Vec2::new(0.5, 2.),
        Vec2::new(0., -8.),
        1.,
    ));

    for a in 0..3600 {
        let dir = Vec2::from_angle((a as f32 / 10.).to_radians());
        sim.spawn(RigidBody::new(
            Shape::Sphere { radius: 0.05 },
            0.5,
            dir * 100.,
            -dir * 5.,
            1.,
        ));
    }

    let scale = 10.;
    let delta_step = 0.1;
    let draw_vel = false;
    let mut frames = 0;
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
        sim.tick(0.05);
        frames += 1;

        if frames >= 500 {
            break;
        }

        next_frame().await
    }
}
