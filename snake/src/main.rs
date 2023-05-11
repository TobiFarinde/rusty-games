mod components;
mod constants;

use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowResolution},
};
use components::{Direction, Position, Size, SnakeHead};
use constants::{
    ARENA_HEIGHT, ARENA_WIDTH, BACKGROUND_COLOR, SNAKE_SEGMENT_COLOR,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                resolution: WindowResolution::new(500., 500.),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FixedTime::new_from_secs(0.150))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(snake_movement_input.before(snake_movement))
        .add_systems(
            (position_translation, size_scaling)
                .chain()
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(
            snake_movement.run_if(on_timer(Duration::from_secs_f32(0.150))),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        },
        SnakeHead {
            direction: Direction::Up,
        },
        Position { x: 3, y: 3 },
        Size::square(0.8),
    ));
}

fn snake_movement(
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            head_pos.x = 3;
            head_pos.y = 3;
        }
    }
}

fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
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
            0.0,
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
