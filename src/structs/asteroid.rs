use crate::{
    Vec2, vec2, screen_width, screen_height,
    polar_to_cartesian, cartesian_to_polar,
    draw_circle
};
use macroquad::ui::hash;
use super::params::AsteroidParams;

use rand::Rng;

#[derive(Clone)]
pub struct Asteroid {
    id: u64,
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    params: AsteroidParams,
}

impl Asteroid {

    pub fn new_random(params: AsteroidParams) -> Self {
        let mut rng = rand::thread_rng();
        let pos = vec2(
            rng.gen_range(1.0..screen_width()),
            rng.gen_range(1.0..screen_width())
        );
        let vel = vec2(
            rng.gen_range(params.vel_range.clone()),
            rng.gen_range(params.vel_range.clone())
        );
        let radius = rng.gen_range(params.size_range.clone());
        Self {
            id: hash!(),
            pos,
            vel,
            radius,
            params: params.clone(),
        }
    }

    pub fn get_child(&self) -> Asteroid {
        let mut rng = rand::thread_rng();
        let mut child = Self {
            id: hash!(),
            pos: self.pos,
            vel: self.vel,
            radius: self.radius * rng.gen_range(self.params.child_radius_variation.clone()),
            params: self.params.clone(),
        };
        let (_, heading) = cartesian_to_polar(self.vel).into();
        child.set_heading(
            heading * rng.gen_range(self.params.child_heading_variation.clone())
        );
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
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.radius,
            self.params.draw.color
        )
    }

    pub fn is_point_overlapping(&self, p: Vec2) -> bool {
        self.pos.distance(p) < self.radius
    }

    fn is_too_small_to_split(&self) -> bool {
        self.radius < self.params.min_radius_to_split
    }

    pub fn set_heading(&mut self, angle: f32) {
        polar_to_cartesian(self.vel.length(), angle);
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Asteroid) -> bool {
        self.pos == other.pos  // id comparison
    }

    fn ne(&self, other: &Asteroid) -> bool {
        self.pos != other.pos
    }
}
