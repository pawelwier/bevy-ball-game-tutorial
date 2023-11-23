pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .init_resource::<HighScores>()
    .init_resource::<StarSpawnTimer>()
    .init_resource::<EnemySpawnTimer>()
    .add_event::<GameOver>()
    .add_systems(Startup, (
        spawn_player, spawn_camera, spawn_enemies, spawn_stars
    ))
    .add_systems(Update, (
        player_movement, enemy_movement, confine_player_movement, confine_enemy_movement, 
        update_enemy_direction, enemy_hit_player, player_hit_star, update_score,
        tick_star_spawn_timer, tick_enemy_spawn_timer, spawn_stars_over_time, spawn_enemies_over_time,
        exit_game, handle_game_over, update_highscores, high_scores_updated
    ))
    .run();
}