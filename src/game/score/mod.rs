use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::HighScores;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(Update, update_score
                .run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, (update_highscores, high_scores_updated))
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}