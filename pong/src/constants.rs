use bevy::{
    math::{Vec2, Vec3},
    render::color::Color,
};

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
pub const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
pub const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
pub const PADDLE_PADDING: f32 = 10.0;
// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
pub const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
pub const BALL_SPEED: f32 = 400.0;
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

pub const WALL_THICKNESS: f32 = 10.0;
// x coordinates
pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
// y coordinates
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
