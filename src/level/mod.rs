use bevy::{prelude::*, core::FixedTimestep};

mod spawner;

use spawner::*;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SpawnData>()
      .add_startup_system(level_startup)
      .add_system_set(
        // TODO: fixed timestep with game state not working apparently
        // change it to a system: time_passed += delta; time_passed >= 6.
        SystemSet::on_update(GameState::Running)
          .with_run_criteria(FixedTimestep::step(6.))
          .with_system(spawn_enemies)
      );
  }
}

fn level_startup (
  mut commands: Commands,
) {
  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("c5ccb8").unwrap(),
        custom_size: Some(Vec2::new(960., 720.)),
        ..Default::default()
      },
      ..Default::default()
    }
  );
}
