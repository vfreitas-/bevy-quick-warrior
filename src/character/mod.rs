use crate::GameState;
use bevy::prelude::*;

mod collisions;
use collisions::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(health_invincible)
          .with_system(kill_enemy)
          .with_system(damage)
      );
  }
}

#[derive(Component, Debug)]
pub struct Health {
  pub max_health: usize,
  pub health: usize,
  pub is_invincible: bool,
  pub invincible_duration: Timer,
}

impl Health {
  pub fn from_health (health: usize) -> Self {
    Self  {
      max_health: health,
      health: health,
      is_invincible: false,
      invincible_duration: Timer::from_seconds(5., false),
    }
  }

  pub fn decrease_health(&mut self, value: usize) {
    if self.is_invincible {
      return;
    }

    if self.health == 0 {
      return;
    }

    if self.health - value > self.max_health {
      // do something maybe?
      return;
    }

    self.health -= value;
    self.is_invincible = true;
  }
}

#[derive(Component, Default)]
pub struct Hitbox {
  pub damage: usize,
}

fn health_invincible (
  time: Res<Time>,
  mut query: Query<&mut Health>,
) {
  for mut health in query.iter_mut() {
    if !health.is_invincible {
      return;
    }

    health.invincible_duration.tick(time.delta());

    if health.invincible_duration.finished() {
      health.is_invincible = false;
      health.invincible_duration.reset();
    }
  }
}
