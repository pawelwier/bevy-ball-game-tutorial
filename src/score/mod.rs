use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::{HighScores, Score};
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, (update_score, update_highscores, high_scores_updated));
    }
}