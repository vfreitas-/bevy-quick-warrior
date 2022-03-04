use bevy::prelude::*;
use benimator::*;
use core::time::Duration;

#[derive(Default)]
pub struct PlayerAnimations {
  pub attack_y: Handle<SpriteSheetAnimation>,
  pub attack_x: Handle<SpriteSheetAnimation>,
}

pub fn create_animations (
  mut handles: ResMut<PlayerAnimations>,
  mut assets: ResMut<Assets<SpriteSheetAnimation>>,
) {
  handles.attack_y = assets.add(SpriteSheetAnimation::from_range(
    0..=3,
    Duration::from_millis(100),
  ).once());
  handles.attack_x = assets.add(SpriteSheetAnimation::from_range(
    4..=7,
    Duration::from_millis(100),
  ).once());
}
