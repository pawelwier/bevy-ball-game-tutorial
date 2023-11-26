use bevy::prelude::*;

pub mod enemy;
mod player;
mod score;
mod star;
pub mod helpers;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app 
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default] 
    Paused
}