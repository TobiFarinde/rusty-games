use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{Deref, DerefMut},
};

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;
