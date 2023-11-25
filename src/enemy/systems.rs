use bevy::{prelude::*, window::PrimaryWindow, audio::VolumeLevel};
use rand::random;

use crate::utils::helpers::confine_movement;

use super::components::*;
use super::resources::EnemySpawnTimer;
use super::{NUMBER_OF_ENEMIES, ENEMY_SIZE, ENEMY_SPEED};

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

pub fn enemy_movement(
  mut enemy_query: Query<(&mut Transform, &Enemy)>,
  time: Res<Time>
) {
  for (mut transform, enemy) in enemy_query.iter_mut() {
      let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
      transform.translation += direction * ENEMY_SPEED * time.delta_seconds()
  } 
}

pub fn confine_enemy_movement(
  mut enemy_query: Query<&mut Transform>,
  window_query: Query<&Window, With<PrimaryWindow>>
) {
  let window = window_query.get_single().unwrap();

  for transform in enemy_query.iter_mut() {
      confine_movement(transform, window, ENEMY_SIZE);
  }
}

fn make_bump_sound(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  let effect_path = if random::<f32>() > 0.5 { "audio/pluck_001.ogg" } else { "audio/pluck_002.ogg" };

  commands.spawn(
      AudioBundle {
          source: asset_server.load(effect_path),
          settings: PlaybackSettings { 
              mode: bevy::audio::PlaybackMode::Remove,
              volume: {bevy::audio::Volume::Absolute(VolumeLevel::new(0.2))},
              ..default()
          }
      },
  );
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

pub fn tick_enemy_spawn_timer(
  mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
  time: Res<Time>
) {
  enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  enemy_spawn_timer: Res<EnemySpawnTimer>
) {
  if enemy_spawn_timer.timer.finished() {
      let window = window_query.get_single().unwrap();
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