use crate::{GameState, player::Player};
use crate::utils::vec2::*;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use heron::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(enemy_follow_player);
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

fn enemy_follow_player (
  time: Res<Time>,
  mut query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
  mut q2: Query<(&mut Velocity, &Transform), (With<Enemy>, Without<Player>)>,
) {

  if let Some(player_transform) = query.iter_mut().next() {
    for (mut velocity, transform) in q2.iter_mut() {

      let direction = direction_to(
        transform.translation.xy(),
        player_transform.translation.xy()
      );

      let input_velocity = direction * 2000. * time.delta_seconds();
      velocity.linear = Vec3::new(input_velocity.x, input_velocity.y, 1.);
    }
  }
}
