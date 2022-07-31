use crate::{
    draw_triangle,
    Vec2, vec2,
    ShipParams,
};
use macroquad::math;
use super::{
    params::ShipDrawParams,
    utils::to_screen_cord,
};

pub struct Ship {
    pos: Vec2,
    vel: f32,
    acc: f32,
    heading: f32,
    small_radius: f32,
    big_radius: f32,
    pub turn_speed: f32,
    draw_params: ShipDrawParams,
}

impl Ship {
    pub fn new(params: ShipParams) -> Self {
        Self {
            pos: params.ship_start_position,
            vel: params.ship_start_velocity,
            acc: params.ship_acceleration,
            heading: 270.,
            small_radius: params.default_small_radius,
            big_radius: params.default_big_radius,
            turn_speed: params.default_turn_speed,
            draw_params: params.draw,
        }
    }

    pub fn update_self(&mut self) {
        let a = math::polar_to_cartesian(self.vel, self.heading.to_radians());
        self.pos += a
    }

    pub fn accelerate(&mut self, factor: f32) {
        self.vel += self.acc * factor;
    }

    pub fn draw_self(&self) {
        let (a, b, c) = self.calc_corners();
        draw_triangle(
            to_screen_cord(a),
            to_screen_cord(b),
            to_screen_cord(c),
            self.draw_params.color
        )
    }

    fn calc_corners(&self) -> (Vec2, Vec2, Vec2) {
        // given: r & angle
        // x, y
        // cos(angle) = x/r
        // x: r*cos(angle)
        // y: r*sin(angle)

        (
            self.pos + math::polar_to_cartesian(self.small_radius, (self.heading + 120.).to_radians()),
            self.pos + math::polar_to_cartesian(self.small_radius, (self.heading - 120.).to_radians()),
            self.pos + math::polar_to_cartesian(self.big_radius, self.heading.to_radians()),
        )
    }

    pub fn turn(&mut self, angle: f32) {
        self.heading += angle;
        self.heading %= 360.;
        println!["{}", self.heading];
    }
}
