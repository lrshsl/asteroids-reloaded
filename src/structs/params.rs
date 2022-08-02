use crate::Vec2;
use super::utils::to_screen_cord;
use macroquad::math::vec2;


pub struct GameParams {
    pub ship: ShipParams,
    pub asteroid: AsteroidParams,
    pub asteroid_spawning_rate: i32,
}

#[derive(Clone)]
pub struct ShipParams {
    pub ship_start_position: Vec2,
    pub ship_start_velocity: f32,
    pub ship_acceleration: f32,
    pub default_small_radius: f32,
    pub default_big_radius: f32,
    pub default_turn_speed: f32,
    pub air_drag: f32,
}

impl ShipParams {
    pub fn default() -> Self {
        Self {
            ship_start_position: to_screen_cord(vec2(500., 500.)),
            ship_start_velocity: 0.1,
            ship_acceleration: 0.1,
            default_small_radius: 10.,
            default_big_radius: 16.,
            default_turn_speed: 1.8,
            air_drag: 0.99,
        }
    }
}

#[derive(Clone)] // Efficient?
pub struct AsteroidParams {
    pub spawning_rate: f32,
    pub vel_range: std::ops::Range<f32>,
    pub size_range: std::ops::Range<f32>,
    pub min_radius_to_split: f32,
    pub child_radius_variation: std::ops::Range<f32>,
    pub child_heading_variation: std::ops::Range<f32>,
}

impl AsteroidParams {

    pub fn default() -> Self {
        Self {
            spawning_rate: 0.97,
            vel_range: 1.0 .. 3.0,
            size_range: 12.0 .. 30.0,
            min_radius_to_split: 10.,
            child_radius_variation: 0.3 .. 0.6,
            child_heading_variation: 30.0 .. 90.0,
        }
    }
}

