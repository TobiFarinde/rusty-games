mod components;
mod constants;

use bevy::prelude::*;
use components::Snake;
use constants::{
    BACKGROUND_COLOR, BOTTOM_WALL, GAP_BETWEEN_SNAKE_AND_FLOOR, SNAKE_COLOR,
    SNAKE_SIZE_HORIZONTAL, SNAKE_SIZE_VERTICAL,
};
use shared::{Collider, WallBundle, WallLocation};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(move_snake)
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
                scale: SNAKE_SIZE_HORIZONTAL,
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

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Snake>>,
) {
    let mut box_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        box_transform.translation.x -= 1.;
        box_transform.scale = SNAKE_SIZE_HORIZONTAL;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        box_transform.translation.x += 1.;
        box_transform.scale = SNAKE_SIZE_HORIZONTAL;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        box_transform.translation.y += 1.;
        box_transform.scale = SNAKE_SIZE_VERTICAL;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        box_transform.translation.y -= 1.;
        box_transform.scale = SNAKE_SIZE_VERTICAL;
    }
}
