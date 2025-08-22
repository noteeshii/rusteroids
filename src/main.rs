mod asteroid;
mod game;
mod ship;

use raylib::prelude::*;

use asteroid::Asteroid;
use game::Game;
use ship::{Bullet, Ship};

pub const WINDOW_X: i32 = 640;
pub const WINDOW_Y: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_X, WINDOW_Y)
        .title("rusteroids")
        .log_level(TraceLogLevel::LOG_NONE)
        .build();

    rl.set_target_fps(60);

    let mut rng = rand::rng();

    let mut game = Game::new();

    while !rl.window_should_close() {
        game.update(&rl, &mut rng);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        game.draw(&mut d);
    }
}
