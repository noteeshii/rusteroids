use std::ops::Add;

use rand::Rng;
use rand::rngs::ThreadRng;
use raylib::prelude::*;

use crate::{WINDOW_X, WINDOW_Y};

#[derive(Clone)]
pub enum AsteroidSize {
    LARGE,
    MEDIUM,
    SMALL,
}

impl AsteroidSize {
    pub fn size_scale(&self) -> f32 {
        match self {
            AsteroidSize::LARGE => 3.,
            AsteroidSize::MEDIUM => 2.,
            AsteroidSize::SMALL => 1.,
        }
    }

    pub fn speed_scale(&self) -> f32 {
        match self {
            AsteroidSize::LARGE => 0.5,
            AsteroidSize::MEDIUM => 1.,
            AsteroidSize::SMALL => 1.5,
        }
    }

    pub fn rotation_scale(&self) -> f32 {
        match self {
            AsteroidSize::LARGE => 1.,
            AsteroidSize::MEDIUM => 2.,
            AsteroidSize::SMALL => 3.,
        }
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        let num = rng.random_range(0..3);

        match num {
            0 => AsteroidSize::LARGE,
            2 => AsteroidSize::SMALL,
            _ => AsteroidSize::MEDIUM,
        }
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub size: AsteroidSize,
    pub position: Vector2,
    pub rotation: f32,
    pub velocity: Vector2,
    pub points: Vec<Vector2>,
}

impl Asteroid {
    pub fn random(rng: &mut ThreadRng) -> Self {
        let points = vec![
            Vector2::new(rng.random_range(-1.0..=1.0), rng.random_range(-7.5..=-6.5)),
            Vector2::new(rng.random_range(3.5..=6.5), rng.random_range(-5.5..=-4.5)),
            Vector2::new(rng.random_range(5.5..=8.5), rng.random_range(-1.0..=1.0)),
            Vector2::new(rng.random_range(3.5..=6.5), rng.random_range(3.5..=6.5)),
            Vector2::new(rng.random_range(-1.0..=1.0), rng.random_range(5.5..=8.5)),
            Vector2::new(rng.random_range(-7.5..=-4.5), rng.random_range(3.5..=6.5)),
            Vector2::new(rng.random_range(-9.5..=-6.5), rng.random_range(-1.0..=1.0)),
            Vector2::new(rng.random_range(-7.5..=-4.5), rng.random_range(-7.5..=-4.5)),
        ];
        let position = Vector2::new(
            rng.random_range(0.0..WINDOW_X as f32),
            rng.random_range(0.0..WINDOW_Y as f32),
        );
        let velocity = Vector2::new(rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0));

        Self {
            size: AsteroidSize::random(rng),
            position,
            rotation: 0.,
            velocity,
            points,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let len = self.points.len();

        for i in 0..len {
            let start = self.points[i]
                .rotated(self.rotation)
                .scale_by(self.size.size_scale())
                .add(self.position);
            let end = (if i + 1 == len {
                self.points[0]
            } else {
                self.points[i + 1]
            })
            .rotated(self.rotation)
            .scale_by(self.size.size_scale())
            .add(self.position);

            d.draw_line(
                start.x as i32,
                start.y as i32,
                end.x as i32,
                end.y as i32,
                Color::WHITE,
            );
        }
    }

    pub fn moving(&mut self) {
        self.position = self
            .position
            .add(self.velocity.scale_by(self.size.speed_scale()));
        self.position = Vector2::new(
            self.position.x.rem_euclid(WINDOW_X as f32),
            self.position.y.rem_euclid(WINDOW_Y as f32),
        );
        self.rotation += self.size.rotation_scale() * 0.01;
    }

    pub fn to_size(&self, size: AsteroidSize) -> Self {
        Self {
            size,
            ..self.clone()
        }
    }

    pub fn destroy(&self) -> Option<(Asteroid, Asteroid)> {
        let size = match self.size {
            AsteroidSize::LARGE => AsteroidSize::MEDIUM,
            AsteroidSize::MEDIUM => AsteroidSize::SMALL,
            AsteroidSize::SMALL => {
                return None;
            }
        };
        let mut l = self.to_size(size.clone());
        let mut r = self.to_size(size);

        l.velocity = l.velocity.rotated(45.);
        r.velocity = r.velocity.rotated(-45.);

        Some((l, r))
    }
}
