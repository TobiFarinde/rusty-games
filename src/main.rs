use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
        ..default()
    });

    // rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::MAROON,
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
        ..default()
    });
}
