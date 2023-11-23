use bevy::ecs::component::Component;
use bevy::math::Vec2;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2
}

#[derive(Component)]
pub struct Star {}