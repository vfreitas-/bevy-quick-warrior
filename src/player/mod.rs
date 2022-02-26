use crate::{GameState};
use bevy::prelude::*;
use heron::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(player_setup)
      .add_system_set(
        // TODO: improve order with labels
        SystemSet::on_update(GameState::Running)
          .with_system(player_input)
          .with_system(player_movement)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::Running)
          .with_system(player_pause)
      );
  }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerMovement {
  velocity: Vec2,
}

#[derive(Component)]
pub struct PlayerDash;

#[derive(Component)]
pub struct PlayerAttack;

fn player_setup(
  mut commands: Commands,
) {
  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("68aca9").unwrap(),
        custom_size: Some(Vec2::new(16., 16.)),
        ..Default::default()
      },
      ..Default::default()
    }
  )
  .insert(RigidBody::KinematicVelocityBased)
  .insert(CollisionShape::Sphere { radius: 0.75 })
  .insert(Velocity::from_linear(Vec3::ZERO))
  .insert(Acceleration::from_linear(Vec3::ZERO))
  .insert(Player)
  .insert(PlayerMovement::default());
}

fn player_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut commands: Commands,
  mut query: Query<(Entity, &mut PlayerMovement), With<Player>>,
) {
  if let Some((entity, mut player_movement)) = query.iter_mut().next() {
    if keyboard_input.pressed(KeyCode::Space) {
      // insert dash component
      commands.entity(entity).insert(PlayerDash);
    }
    if keyboard_input.pressed(KeyCode::F) {
      // insert attack component
      commands.entity(entity).insert(PlayerAttack);
    }

    let mut input_velocity = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
      input_velocity.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
      input_velocity.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::W) {
      input_velocity.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
      input_velocity.y -= 1.;
    }
  
    input_velocity = input_velocity.normalize_or_zero();
    player_movement.velocity = input_velocity;
  }
}

fn player_movement(
  time: Res<Time>,
  mut query: Query<(&mut Velocity, &PlayerMovement), With<Player>>,
) {
  if let Some((mut velocity, player_movement)) = query.iter_mut().next() {
    let input_velocity = player_movement.velocity * 5000. * time.delta_seconds();
    velocity.linear = Vec3::new(input_velocity.x, input_velocity.y, 1.);
  }
}

// fn player_dash() {}

// fn player_attack () {}

fn player_pause (
  mut query: Query<(&mut Velocity, &mut PlayerMovement), With<Player>>,
) {
  if let Some((mut velocity, mut player_movement)) = query.iter_mut().next() {
    velocity.linear = Vec3::ZERO;
    player_movement.velocity = Vec2::ZERO;
  }
}
