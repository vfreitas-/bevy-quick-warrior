use bevy::prelude::*;

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
        SystemSet::on_update(GameState::Running)
          .with_system(spawn_enemies_constant)
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
