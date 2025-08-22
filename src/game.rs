use std::collections::HashSet;
use std::ops::Add;

use rand::rngs::ThreadRng;
use raylib::prelude::*;

use crate::{Asteroid, Bullet, Ship};
use crate::{WINDOW_X, WINDOW_Y};

#[derive(PartialEq)]
enum State {
    LOOSE,
    NONE,
}

pub struct Game {
    ship: Ship,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
    state: State,
    score: u32,
    wave: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ship: Ship::default(),
            asteroids: vec![],
            bullets: vec![],
            state: State::NONE,
            score: 0,
            wave: 0,
        }
    }

    fn random_asteroids(&mut self, count: u32, rng: &mut ThreadRng) {
        for _ in 0..count {
            self.asteroids.push(Asteroid::random(rng));
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.state == State::LOOSE {
            d.draw_text(
                "You lose!",
                (WINDOW_X / 2) - 95,
                (WINDOW_Y / 2) - 40,
                40,
                Color::RED,
            );
            d.draw_text(
                "Press `r` to restart",
                (WINDOW_X / 2) - 110,
                WINDOW_Y / 2,
                20,
                Color::WHITE,
            );
        }

        d.draw_text(&format!("Score: {}", self.score), 10, 10, 15, Color::WHITE);
        d.draw_text(&format!("Wave: {}", self.wave), 10, 25, 15, Color::WHITE);

        self.ship.draw(d);

        for a in self.asteroids.iter() {
            a.draw(d);
        }
        for b in self.bullets.iter() {
            b.draw(d);
        }
    }

    pub fn handle_input(&mut self, rl: &RaylibHandle) {
        if self.state == State::LOOSE {
            if rl.is_key_pressed(KeyboardKey::KEY_R) {
                *self = Game::new();
            }

            return;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.ship.rotate_by(-0.1);
        } else if rl.is_key_down(KeyboardKey::KEY_D) {
            self.ship.rotate_by(0.1);
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.ship.accelerate_by(-0.1);
        } else if rl.is_key_down(KeyboardKey::KEY_S) {
            self.ship.accelerate_by(0.1);
        } else {
            self.ship.stopping();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.bullets.push(self.ship.shoot());
        }
    }

    fn collision_asteroid_with_bullet(asteroid: &Asteroid, bullet: &Bullet) -> bool {
        let len = asteroid.points.len();

        for i in 0..len {
            let start = asteroid.points[i]
                .rotated(asteroid.rotation)
                .scale_by(asteroid.size.size_scale())
                .add(asteroid.position);

            if asteroid.position.distance_to(bullet.position)
                <= asteroid.position.distance_to(start)
            {
                return true;
            }
        }

        return false;
    }

    fn collision_asteroid_with_ship(asteroid: &Asteroid, ship: &Ship) -> bool {
        let len = asteroid.points.len();

        for i in 0..len {
            let start = asteroid.points[i]
                .rotated(asteroid.rotation)
                .scale_by(asteroid.size.size_scale())
                .add(asteroid.position);

            if asteroid.position.distance_to(ship.position) <= asteroid.position.distance_to(start)
            {
                return true;
            }
        }

        return false;
    }

    pub fn update(&mut self, rl: &RaylibHandle, rng: &mut ThreadRng) {
        self.handle_input(rl);

        self.ship.moving();

        if self.state == State::LOOSE {
            return;
        }
        if self.state == State::NONE && self.asteroids.is_empty() {
            self.random_asteroids(self.wave + 5, rng);
            self.wave += 1;
        }

        let mut asteroid_indexes = HashSet::new();
        let mut bullet_indexes = HashSet::new();

        for asteroid_idx in 0..self.asteroids.len() {
            let asteroid = &self.asteroids[asteroid_idx];

            if Game::collision_asteroid_with_ship(asteroid, &self.ship) {
                self.state = State::LOOSE;
                break;
            }

            for bullet_idx in 0..self.bullets.len() {
                let bullet = &self.bullets[bullet_idx];

                if Game::collision_asteroid_with_bullet(asteroid, bullet) {
                    asteroid_indexes.insert(asteroid_idx);
                    bullet_indexes.insert(bullet_idx);
                }
            }
        }

        for i in asteroid_indexes {
            let a = self.asteroids.remove(i);

            self.score += 10 * a.size.rotation_scale() as u32;

            if let Some((l, r)) = a.destroy() {
                self.asteroids.push(l);
                self.asteroids.push(r);
            }
        }
        for i in bullet_indexes {
            self.bullets.remove(i);
        }

        self.bullets.retain(|b| !b.is_out());

        for a in self.asteroids.iter_mut() {
            a.moving();
        }
        for b in self.bullets.iter_mut() {
            b.moving();
        }
    }
}
