use crate::{
    color_u8, draw_triangle, screen_height, screen_width, Asteroid, Color, ShipParams, Vec2,
};
use macroquad::math;

pub struct Ship {
    pos: Vec2,
    vel: f32,
    acc: f32,
    air_drag: f32,
    heading: f32,
    small_radius: f32,
    big_radius: f32,
    pub turn_speed: f32,
    color: Color,
}

impl Ship {
    pub fn new(params: ShipParams) -> Self {
        Self {
            pos: params.ship_start_position,
            vel: params.ship_start_velocity,
            acc: params.ship_acceleration,
            air_drag: params.air_drag,
            heading: 270.,
            small_radius: params.default_small_radius,
            big_radius: params.default_big_radius,
            turn_speed: params.default_turn_speed,
            color: color_u8![255., 255., 255., 255.],
        }
    }

    pub fn update_self(&mut self) {
        self.vel *= self.air_drag;
        let a = math::polar_to_cartesian(self.vel, self.heading.to_radians());
        self.pos += a;
        self.wrap_around()
    }

    fn wrap_around(&mut self) {
        if self.pos.x < 0. || self.pos.x > screen_width() {
            self.heading -= 90.;
            self.heading *= -1.;
            self.heading += 90.;
        } else if self.pos.y < 0. || self.pos.y > screen_height() {
            self.heading *= -1.
        }
    }

    pub fn accelerate(&mut self, factor: f32) {
        self.vel += self.acc * factor;
    }

    pub fn draw_self(&self) {
        let (a, b, c) = self.calc_corners();
        draw_triangle(a, b, c, self.color)
    }

    fn calc_corners(&self) -> (Vec2, Vec2, Vec2) {
        (
            self.pos
                + math::polar_to_cartesian(self.small_radius, (self.heading + 120.).to_radians()),
            self.pos
                + math::polar_to_cartesian(self.small_radius, (self.heading - 120.).to_radians()),
            self.pos + math::polar_to_cartesian(self.big_radius, self.heading.to_radians()),
        )
    }

    pub fn turn(&mut self, angle: f32) {
        self.heading += angle;
        self.heading %= 360.;
    }

    pub fn is_overlapping(&self, ast: &Asteroid) -> bool {
        let (a, b, c) = self.calc_corners();
        ast.is_point_overlapping(a) && ast.is_point_overlapping(b) && ast.is_point_overlapping(c)
    }

    pub fn collide(&mut self, ast: &Asteroid) {
        self.color = ast.color;
    }
}
