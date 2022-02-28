use bevy::prelude::*;
use benimator::*;
use core::time::Duration;

#[derive(Default)]
pub struct PlayerAnimations {
  pub attack: Handle<SpriteSheetAnimation>,
}

pub fn create_animations (
  mut handles: ResMut<PlayerAnimations>,
  mut assets: ResMut<Assets<SpriteSheetAnimation>>,
) {
  handles.attack = assets.add(SpriteSheetAnimation::from_range(
    0..=3,
    Duration::from_millis(100),
  ).once());
}
