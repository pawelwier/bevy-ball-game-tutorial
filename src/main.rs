use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies))
    .add_systems(Update, (player_movement, confine_movement))
    .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {}
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let (random_x, random_y) = (random::<f32>() * window.width(), random::<f32>() * window.height());

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {}
            )
        );
    }
}

pub fn key_from_pair_pressed (
    keyboard_input: &Res<Input<KeyCode>>,
    keycodes: [KeyCode; 2]
) -> bool {
    let mut is_pressed = false;
    for code in keycodes {
        if keyboard_input.pressed(code) { is_pressed = true; }
    }
    is_pressed
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let (mut x, mut y) = (0.0, 0.0);

        if key_from_pair_pressed(&keyboard_input, [KeyCode::Left, KeyCode::A]) { x = -1.0 }
        if key_from_pair_pressed(&keyboard_input, [KeyCode::Right, KeyCode::D]) { x = 1.0 }
        if key_from_pair_pressed(&keyboard_input, [KeyCode::Up, KeyCode::W]) { y = 1.0 }
        if key_from_pair_pressed(&keyboard_input, [KeyCode::Down, KeyCode::S]) { y = -1.0 }

        if x != 0.0 || y != 0.0 {
            direction += Vec3::new(x, y, 0.0);
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_size = PLAYER_SIZE / 2.0;
        let mut translation = transform.translation;

        let x_max = window.width() - half_size;
        let y_max = window.height() - half_size;
        if translation.x >= x_max { translation.x = x_max; }
        if translation.x <= half_size { translation.x = half_size }
        if translation.y >= y_max { translation.y = y_max; }
        if translation.y <= half_size { translation.y = half_size }

        transform.translation = translation;
    }
}