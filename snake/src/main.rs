mod components; mod constants;

use bevy::{prelude::*, window::PrimaryWindow};
use components::{SnakeHead, Size, Position};
use constants::{
    BACKGROUND_COLOR,
    SNAKE_SIZE, ARENA_WIDTH, ARENA_HEIGHT,
    SNAKE_HEAD_COLOR, SNAKE_SEGMENT_COLOR,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor { 
        width: 140.0,
        height:140.0,
        title: "Game of Life".to_string(),
        ..default()
      },
      ..default()
    }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(move_snake)
        .add_system(size_scaling)
        .add_system(position_translation)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: SNAKE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        },
        SnakeHead,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        },
        SnakeHead,
        Position {x: 3, y: 3 },
        Size::square(0.8)
    ));
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Position, With<SnakeHead>>,
) {

    for mut pos in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
    }

}

fn size_scaling(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &Size)>,
) {
    let window = window.single();

    for (mut transform, sprite_size) in &mut query {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &Position)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = window.single();
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
