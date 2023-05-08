use bevy::{
    math::Vec3,
    render::color::Color,
};


// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
pub const SNAKE_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);

pub const ARENA_WIDTH: u32 = 25;
pub const ARENA_HEIGHT: u32 = 25;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
