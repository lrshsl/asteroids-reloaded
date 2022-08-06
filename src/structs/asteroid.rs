use super::params::AsteroidParams;
use crate::{
    cartesian_to_polar, color_u8, draw_circle, polar_to_cartesian, screen_height, screen_width,
    vec2, Color, Vec2,
};

use rand::Rng;

#[derive(Clone)]
pub struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    pub color: Color,
    params: AsteroidParams,
}

impl Asteroid {
    pub fn new_random(params: AsteroidParams) -> Self {
        let mut rng = rand::thread_rng();
        let pos = vec2(
            rng.gen_range(1.0..screen_width()),
            rng.gen_range(1.0..screen_width()),
        );
        let vel = vec2(
            rng.gen_range(params.vel_range.clone()),
            rng.gen_range(params.vel_range.clone()),
        );
        let radius = rng.gen_range(params.size_range.clone());
        let color = color_u8![
            rng.gen_range(0.0..255.0),
            rng.gen_range(0.0..255.0),
            rng.gen_range(0.0..255.0),
            255.
        ];
        Self {
            pos,
            vel,
            radius,
            color,
            params,
        }
    }

    pub fn get_child(&self) -> Asteroid {
        let mut rng = rand::thread_rng();
        let mut child = Self {
            pos: self.pos,
            vel: self.vel,
            radius: self.radius * rng.gen_range(self.params.child_radius_variation.clone()),
            color: self.color,
            params: self.params.clone(),
        };
        let (_, heading) = cartesian_to_polar(self.vel).into();
        child.set_heading(heading * rng.gen_range(self.params.child_heading_variation.clone()));
        child
    }

    pub fn update_self(&mut self) {
        self.pos += self.vel;
        if self.pos.x > screen_width() {
            self.pos.x = 0.
        } else if self.pos.x < 0. {
            self.pos.x = screen_width()
        } else if self.pos.y > screen_height() {
            self.pos.y = 0.
        } else if self.pos.y < 0. {
            self.pos.y = screen_height()
        }
    }

    pub fn draw_self(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color)
    }

    pub fn is_point_overlapping(&self, p: Vec2) -> bool {
        self.pos.distance(p) < self.radius
    }

    pub fn set_heading(&mut self, angle: f32) {
        polar_to_cartesian(self.vel.length(), angle);
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Asteroid) -> bool {
        self.pos == other.pos // id comparison?
    }
}
