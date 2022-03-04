use bevy::prelude::*;

mod spawner;

use spawner::*;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SpawnData>()
      .add_system_to_stage(CoreStage::PostUpdate, enemy_removed)
      .add_system_set(
        SystemSet::on_enter(GameState::Starting)
          .with_system(level_startup)
          .with_system(spawn_startup)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Starting)
          .with_system(running)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(spawn_enemies_constant)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::GameOver)
          .with_system(level_despawn)
      );
  }
}

#[derive(Component, Debug)]
pub struct LevelBG;

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
  )
  .insert(LevelBG);
}

fn running (mut state: ResMut<State<GameState>>) {
  if &GameState::Starting == state.current() {
    state.set(GameState::Running).unwrap();
  }
}

pub fn level_despawn (
  mut commands: Commands,
  query: Query<Entity, With<LevelBG>>,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
