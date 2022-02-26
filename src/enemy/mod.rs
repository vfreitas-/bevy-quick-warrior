use crate::{GameState};
use bevy::prelude::*;
use heron::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    // app
    //   .add_startup_system(enemy_setup);
  }
}

#[derive(Component)]
pub struct Enemy;

//TODO: merge it with PlayerMovement
#[derive(Component, Default)]
pub struct EnemyMovement {
  velocity: Vec2,
}

pub fn enemy_spawn(
  commands: &mut Commands,
  transform: Transform,
) {
  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("9a4f50").unwrap(),
        custom_size: Some(Vec2::new(16., 16.)),
        ..Default::default()
      },
      transform: transform,
      ..Default::default()
    }
  )
  .insert(RigidBody::KinematicVelocityBased)
  .insert(CollisionShape::Sphere { radius: 0.75 })
  .insert(Velocity::from_linear(Vec3::ZERO))
  .insert(Acceleration::from_linear(Vec3::ZERO))
  .insert(Enemy);
}
