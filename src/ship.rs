use std::f32::consts::PI;
use std::ops::Add;

use raylib::prelude::*;

use crate::{WINDOW_X, WINDOW_Y};

pub struct Ship {
    pub position: Vector2,
    rotation: f32,
    velocity: Vector2,
}

impl Ship {
    pub fn default() -> Self {
        Self {
            position: Vector2::new(WINDOW_X as f32 / 2., WINDOW_Y as f32 / 2.),
            rotation: 0.,
            velocity: Vector2::zero(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_triangle_lines(
            Vector2::new(0., -5.)
                .rotated(self.rotation)
                .scale_by(3.)
                .add(self.position),
            Vector2::new(3., 3.)
                .rotated(self.rotation)
                .scale_by(3.)
                .add(self.position),
            Vector2::new(-3., 3.)
                .rotated(self.rotation)
                .scale_by(3.)
                .add(self.position),
            Color::WHITE,
        );
    }

    pub fn moving(&mut self) {
        self.position = self.position.add(self.velocity);
        self.position = Vector2::new(
            self.position.x.rem_euclid(WINDOW_X as f32),
            self.position.y.rem_euclid(WINDOW_Y as f32),
        );
    }

    pub fn rotate_by(&mut self, angle: f32) {
        self.rotation += angle;
    }

    pub fn accelerate_by(&mut self, acc: f32) {
        let angle = self.rotation + (PI * 0.5);
        let direction = Vector2::new(angle.cos(), angle.sin());

        self.velocity = self.velocity.add(direction.scale_by(acc));
    }

    pub fn stopping(&mut self) {
        if self.velocity != Vector2::zero() {
            self.velocity = self.velocity.scale_by(1.0 - 0.015);
        }
    }

    pub fn shoot(&self) -> Bullet {
        Bullet::new(self.position, self.rotation)
    }
}

pub struct Bullet {
    pub position: Vector2,
    rotation: f32,
}

impl Bullet {
    pub fn new(position: Vector2, rotation: f32) -> Self {
        Self { position, rotation }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let start = Vector2::new(0., 4.)
            .rotated(self.rotation)
            .add(self.position);
        let end = Vector2::new(0., -4.)
            .rotated(self.rotation)
            .add(self.position);

        d.draw_line(
            start.x as i32,
            start.y as i32,
            end.x as i32,
            end.y as i32,
            Color::WHITE,
        );
    }

    pub fn moving(&mut self) {
        let angle = self.rotation + (PI * 0.5);
        let direction = Vector2::new(-angle.cos(), -angle.sin());

        self.position = self.position.add(direction.scale_by(10.));
    }

    pub fn is_out(&self) -> bool {
        self.position.x > WINDOW_X as f32
            || self.position.x < 0.
            || self.position.y > WINDOW_Y as f32
            || self.position.y < 0.
    }
}
