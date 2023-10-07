use raylib::prelude::*;
use rand;
use rand::{random, Rng};

#[derive(Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;

fn main() {
    let mut p0 = Point {x: 0., y: 0.}; // Current point
    let mut p1 = Point {x: 0., y: 0.}; // Next point

    let color = Color::new(0, 255, 0, 255);

    // Setup the window
    let (mut rl, thread) = raylib::init()
        .size(WIDTH , HEIGHT)
        .title("Barnsley Fern")
        .msaa_4x()
        .build();

    // rl.set_target_fps(100);
    let mut i = 0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if i < 2 {
            d.clear_background(Color::WHITE);
            i += 1;
        } else {

            let r = rand::thread_rng().gen_range(0..100);

            if r < 1 {
                p1.x = 0.0;
                p1.y = 0.16 * p0.y;
            } else if r < 86 {
                p1.x = 0.85 * p0.x + 0.04 * p0.y;
                p1.y = -0.04 * p0.x + 0.85 * p0.y + 1.6;
            } else if r < 93 {
                p1.x = 0.2 * p0.x - 0.26 * p0.y;
                p1.y = 0.23 * p0.x + 0.22 * p0.y + 1.6;
            } else {
                p1.x = -0.15 * p0.x + 0.28 * p0.y;
                p1.y = 0.26 * p0.x + 0.24 * p0.y + 0.44;
            }

            let x: i32 = WIDTH / 2 + (p1.x * 75.) as i32;
            let y: i32 = HEIGHT - (p1.y * 75.) as i32;

            d.draw_pixel(x, y, color);
            p0 = p1;
        }
    }
}