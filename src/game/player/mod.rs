use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use crate::AppState;
use super::SimulationState;

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
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, (
                player_movement, confine_player_movement, 
                enemy_hit_player, player_hit_star
            ).in_set(MovementSystemSet).run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}