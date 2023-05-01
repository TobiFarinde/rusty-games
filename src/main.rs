use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
            ..default()
        },
        Direction::Down,
    ));

    // rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::MAROON,
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -230., 0.)),
        ..default()
    });
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut dir, mut transform) in &mut sprite_position {
        match *dir {
            Direction::Up => transform.translation.y += 100. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 100. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *dir = Direction::Down;
        } else if transform.translation.y < -200. {
            *dir = Direction::Up;
        }
    }
}
