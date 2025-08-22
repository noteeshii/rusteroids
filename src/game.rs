use rand::rngs::ThreadRng;
use raylib::prelude::*;

use crate::Asteroid;
use crate::{Bullet, Ship};

pub struct Game {
    pub ship: Ship,
    pub asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ship: Ship::default(),
            asteroids: vec![],
            bullets: vec![],
        }
    }

    pub fn random_asteroids(&mut self, count: i32, rng: &mut ThreadRng) {
        for _ in 0..count {
            self.asteroids.push(Asteroid::random(rng));
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.ship.draw(d);

        for a in self.asteroids.iter() {
            a.draw(d);
        }
        for b in self.bullets.iter() {
            b.draw(d);
        }
    }

    pub fn handle_input(&mut self, rl: &RaylibHandle) {
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

    pub fn update(&mut self) {
        self.ship.moving();

        let mut a_is = Vec::new();
        let mut b_is = Vec::new();

        for a_i in 0..self.asteroids.len() {
            let a = self.asteroids[a_i].clone();

            for b_i in 0..self.bullets.len() {
                let b = self.bullets[b_i].clone();

                if a.check_collision(b.position) {
                    a_is.push(a_i);
                    b_is.push(b_i);
                }
            }
        }

        for i in a_is {
            let a = self.asteroids.remove(i);

            if let Some((l, r)) = a.destroy() {
                self.asteroids.push(l);
                self.asteroids.push(r);
            }
        }
        for i in b_is {
            self.bullets.remove(i);
        }

        self.bullets = self
            .bullets
            .clone()
            .into_iter()
            .filter(|b| !b.is_out())
            .collect();

        for a in self.asteroids.iter_mut() {
            a.moving();
        }
        for b in self.bullets.iter_mut() {
            b.moving();
        }
    }
}
