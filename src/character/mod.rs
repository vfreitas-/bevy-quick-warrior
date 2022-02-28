use crate::physics::Layer;
use crate::{GameState, player::Player};
use crate::utils::vec2::*;
use bevy::prelude::*;
use heron::*;

mod collisions;
use collisions::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(kill_enemy)
      );
    //   .add_system_set(
    //     SystemSet::on_exit(GameState::Running)
    //       .with_system(enemy_pause)
    //   );
  }
}

#[derive(Component)]
pub struct Health {
  max_health: usize,
  health: usize,
}

impl Default for Health {
  fn default () -> Self {
    Self {
      max_health: 0,
      health: 0,
    }
  }
}

#[derive(Component, Default)]
pub struct Hitbox {
  damage: usize,
}
