use bevy::{prelude::*, window::PrimaryWindow};
use bevy::audio::VolumeLevel;

use super::components::Player;
use super::{PLAYER_SIZE, PLAYER_SPEED};
use crate::utils::helpers::confine_movement;
use crate::events::GameOver;
use crate::enemy::{ENEMY_SIZE, components::Enemy};
use crate::score::resources::*;
use crate::star::{STAR_SIZE, components::Star};

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

fn key_from_pair_pressed (
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

pub fn confine_player_movement(
  mut player_query: Query<&mut Transform, With<Player>>,
  window_query: Query<&Window, With<PrimaryWindow>>
) {
  if let Ok(transform) = player_query.get_single_mut() {
      let window = window_query.get_single().unwrap();
      confine_movement(transform, window, PLAYER_SIZE);
  }
}

pub fn enemy_hit_player(
  mut commands: Commands,
  mut game_over_event_writer: EventWriter<GameOver>,
  mut player_query: Query<(Entity, &Transform), With<Player>>,
  enemy_query: Query<&Transform, With<Enemy>>,
  asset_server: Res<AssetServer>,
  score: Res<Score>
) {
  let mut is_collision = false;
  if let Ok((player_entity, player_tranform)) = player_query.get_single_mut() {

      for enemy_transform in enemy_query.iter() {
          let distance = player_tranform.translation.distance(enemy_transform.translation);
          let min_distance = PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0;

          is_collision = distance < min_distance;
          if is_collision { break; }
      }
      if is_collision {
          println!("Oh no! BAM!");
          commands.entity(player_entity).despawn();
          commands.spawn(
              AudioBundle {
                  source: asset_server.load("audio/explosionCrunch_000.ogg"),
                  settings: PlaybackSettings { 
                      mode: bevy::audio::PlaybackMode::Remove,
                      ..default()
                  }
              },
          );
          game_over_event_writer.send(GameOver { score: score.value });
      }
  }
}

pub fn player_hit_star(
  mut commands: Commands,
  player_query: Query<&Transform, With<Player>>,
  star_query: Query<(Entity, &Transform), With<Star>>,
  asset_server: Res<AssetServer>,
  mut score: ResMut<Score>
) {
  if let Ok(player_transform) = player_query.get_single() {
      for (star_entity, star_transform) in star_query.iter() {
          let distance = player_transform.translation.distance(star_transform.translation);
          if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
              score.value += 1;
              commands.entity(star_entity).despawn();
              commands.spawn(
                  AudioBundle {
                      source: asset_server.load("audio/laserLarge_000.ogg"),
                      settings: PlaybackSettings { 
                          mode: bevy::audio::PlaybackMode::Remove,
                          volume: {bevy::audio::Volume::Absolute(VolumeLevel::new(0.5))},
                          ..default()
                      }
                  },
              );
          }
      }
  }
}