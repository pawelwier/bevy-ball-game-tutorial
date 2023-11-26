use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::EnemySpawnTimer;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 6;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(Update, (enemy_movement, confine_enemy_movement, update_enemy_direction, 
                tick_enemy_spawn_timer, spawn_enemies_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}