use bevy::prelude::*;

pub mod events;
mod systems;
mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugins((GamePlugin, MainMenuPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, (
        transition_to_game_state, transition_to_main_menu_state,
        exit_game, handle_game_over, 
    ))
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}