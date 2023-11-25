use bevy::prelude::*;

pub fn confine_movement(
  mut transform: Mut<'_, Transform>,
  window: &Window,
  size: f32
) {
  let half_size = size / 2.0;
  let mut translation = transform.translation;

  let x_max = window.width() - half_size;
  let y_max = window.height() - half_size;
  if translation.x >= x_max { translation.x = x_max; }
  if translation.x <= half_size { translation.x = half_size; }
  if translation.y >= y_max { translation.y = y_max; }
  if translation.y <= half_size { translation.y = half_size; }

  transform.translation = translation;
}