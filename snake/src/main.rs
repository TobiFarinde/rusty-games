mod constants;
mod components;

use bevy::prelude::*;
use constants::{
    BACKGROUND_COLOR, 
    GAP_BETWEEN_SNAKE_AND_FLOOR, 
    BOTTOM_WALL,
    SNAKE_SIZE,
    SNAKE_COLOR
};
use shared::{WallBundle, WallLocation, Collider};
use components::Snake;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    // snake
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_SNAKE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: SNAKE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        },
        Snake,
        Collider,
    ));

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
