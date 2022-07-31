use crate::{
    Vec2, vec2, screen_width, screen_height,
    draw_circle
};
use super::params::AsteroidParams;

use rand::Rng;

pub struct Asteroid {
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
            pos,
            vel,
            radius,
            params: params.clone(),
        }
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

    pub fn is_overlapping(&self, other: &Asteroid) -> bool { false }
    pub fn collide(&self, other: &Asteroid) {}
}
