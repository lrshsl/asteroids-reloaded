use crate::{
    Vec2, vec2,
    screen_width,
};

pub fn to_screen_cord(cord: Vec2) -> Vec2 {
    vec2(
        cord.x * screen_width() / 1000.,
        cord.y * screen_width() / 1000.,
    )
}
