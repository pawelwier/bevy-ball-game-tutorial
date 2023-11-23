use bevy::{prelude::*, window::PrimaryWindow, audio::VolumeLevel, app::AppExit};
use rand::prelude::*;

use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const STAR_SIZE: f32 = 30.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 6;
pub const NUMBER_OF_STARS: usize = 10;

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
              volume: {bevy::audio::Volume::Absolute(VolumeLevel::new(0.2))},
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

pub fn spawn_stars(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>
) {
  let window = window_query.get_single().unwrap();

  for _ in 0..NUMBER_OF_STARS {
      let (random_x, random_y) = (random::<f32>() * window.width(), random::<f32>() * window.height());
      commands.spawn(
          (
              SpriteBundle {
                  transform: Transform::from_xyz(random_x, random_y, 0.0),
                  texture: asset_server.load("sprites/star.png"),
                  ..default()
              },
              Star {}
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

pub fn update_score(score: Res<Score>) {
  if score.is_changed() {
      println!("Score: {}", score.value.to_string());
  }
}

pub fn tick_star_spawn_timer(
  mut star_spawn_timer: ResMut<StarSpawnTimer>,
  time: Res<Time>
) {
  star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  star_spawn_timer: Res<StarSpawnTimer>
) {
  if star_spawn_timer.timer.finished() {
      let window =window_query.get_single().unwrap();
      let (random_x, random_y) = (random::<f32>() * window.width(), random::<f32>() * window.height());
      commands.spawn(
          (
              SpriteBundle {
                  transform: Transform::from_xyz(random_x, random_y, 0.0),
                  texture: asset_server.load("sprites/star.png"),
                  ..default()
              },
              Star {}
          )
      );
  }
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

pub fn exit_game(
  keyboard_input: Res<Input<KeyCode>>,
  mut app_exit_event_writer: EventWriter<AppExit>
) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
      app_exit_event_writer.send(AppExit);
  }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
  for event in game_over_event_reader.read() {
      println!("FINAL SCORE: {}", event.score.to_string());
  }
}

pub fn update_highscores(
  mut game_over_event_reader: EventReader<GameOver>,
  mut high_scores: ResMut<HighScores>
) {
  for event in game_over_event_reader.read() {
      high_scores.scores.push(("Player 1".to_string(), event.score))
  }
}

pub fn high_scores_updated(
  high_scores: Res<HighScores>
) {
  if high_scores.is_changed() {
      println!("High scores: {:?}", high_scores);
  }
}