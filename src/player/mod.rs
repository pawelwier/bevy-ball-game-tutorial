use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, MovementSystemSet.before(ConfinementSystemSet))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet), 
                enemy_hit_player, player_hit_star
            ));
    }
}