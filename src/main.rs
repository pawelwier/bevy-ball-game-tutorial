use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 8;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (
        spawn_player, spawn_camera, spawn_enemies
    ))
    .add_systems(Update, (
        player_movement, enemy_movement, confine_player_movement, confine_enemy_movement, update_enemy_direction
    ))
    .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2
}

pub fn make_bump_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let effect_path = if random::<f32>() > 0.5 { "audio/pluck_001.ogg" } else { "audio/pluck_002.ogg" };

    commands.spawn(
        AudioBundle {
            source: asset_server.load(effect_path),
            settings: PlaybackSettings { 
                mode: bevy::audio::PlaybackMode::Remove,
                ..default()
            }
        },
    );
}

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
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
                }
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
    time: Res<Time>,
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

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds()
    } 
}

pub fn confine_movement(
    mut transform: Mut<'_, Transform>,
    window: &Window
) {
    let half_size = PLAYER_SIZE / 2.0;
    let mut translation = transform.translation;

    let x_max = window.width() - half_size;
    let y_max = window.height() - half_size;
    if translation.x >= x_max { translation.x = x_max; }
    if translation.x <= half_size { translation.x = half_size; }
    if translation.y >= y_max { translation.y = y_max; }
    if translation.y <= half_size { translation.y = half_size; }

    transform.translation = translation;
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        confine_movement(transform, window);
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for transform in enemy_query.iter_mut() {
        confine_movement(transform, window);
    }
}

pub fn update_enemy_direction(
    commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let half_size = ENEMY_SIZE / 2.0;
    let mut is_updated = false;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x <= half_size || translation.x >= window.width() - half_size {
            enemy.direction.x *= -1.0;
            is_updated = true;
        }
        if translation.y <= half_size || translation.y >= window.height() - half_size {
            enemy.direction.y *= -1.0;
            is_updated = true;
        }
    }

    if is_updated { make_bump_sound(commands, asset_server); }
}