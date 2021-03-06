use crate::{GameState, physics::Layer, character::Health, quick_event::OnQuickEventPlayerWin};
use benimator::*;
use bevy::prelude::*;
use heron::*;

use animations::*;

mod animations;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<PlayerAnimations>()
      .add_startup_system_to_stage(
        StartupStage::PreStartup, 
        create_animations
      )
      .add_system(player_on_win)
      .add_system_set(
        SystemSet::on_enter(GameState::Starting)
          .with_system(player_setup)
      )
      .add_system_set(
        // TODO: improve order with labels
        SystemSet::on_update(GameState::Running)
          .with_system(player_input)
          .with_system(player_movement)
          .with_system(player_attack_added)
          .with_system(player_attacking)
          .with_system(player_dash)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::Running)
          .with_system(player_pause)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::GameOver)
          .with_system(player_despawn)
      );
  }
}

#[derive(Component)]
pub struct Player {
  speed: f32,
  attack_duration: f32,
}

impl Default for Player {
  fn default () -> Self {
    Self {
      speed: 5000.,
      attack_duration: 0.4,
    }
  }
}

#[derive(Component, Default)]
pub struct PlayerMovement {
  velocity: Vec2,
  speed: f32,
}

#[derive(Component)]
pub struct PlayerDash {
  speed: f32,
  duration: Timer,
}

impl Default for PlayerDash {
  fn default () -> Self {
    Self {
      speed: 15000.,
      duration: Timer::from_seconds(0.3, false),
    }
  }
}

#[derive(Component, Default)]
pub struct PlayerAttack {
  time_passed: f32,
}

#[derive(Component)]
pub struct PlayerHitbox;

fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  animations: Res<PlayerAnimations>,
  mut textures: ResMut<Assets<TextureAtlas>>,
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
  .insert(RigidBody::Dynamic)
  .insert(CollisionShape::Sphere { radius: 7. })
  .insert(Velocity::from_linear(Vec3::ZERO))
  .insert(Acceleration::from_linear(Vec3::ZERO))
  .insert(RotationConstraints::lock())
  .insert(Player::default())
  .insert(PlayerMovement::default())
  .insert(CollisionLayers::none()
    .with_group(Layer::Player)
    .with_masks(&[Layer::World, Layer::Enemy])
  )
  .insert(Health::from_health(5))
  .with_children(|parent| {

    parent.spawn_bundle(SpriteSheetBundle {
      texture_atlas: textures.add(
        TextureAtlas::from_grid(
          asset_server.load("Art/Character/Attack.png"),
          Vec2::new(20., 20.),
          8,
          1
        )
      ),
      visibility: Visibility {
        is_visible: false
      },
      transform: Transform {
        translation: Vec3::new(0., 16., 1.),
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(animations.attack_x.clone())
    .insert(SensorShape)
    .insert(CollisionShape::Cuboid { 
      half_extends: Vec3::new(10., 7., 1.),
      border_radius: None,
    })
    .insert(CollisionLayers::none()
      .with_masks(&[Layer::Enemy])
    )
    .insert(PlayerHitbox);
  });
}

fn player_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut commands: Commands,
  mut query: Query<(
    Entity, 
    &mut PlayerMovement, 
    &Player, 
    Option<&PlayerAttack>,
    Option<&PlayerDash>,
  ), With<Player>>,
) {
  if let Some((
    entity, 
    mut player_movement,
    player,
    attacking,
    dashing
  )) = query.iter_mut().next() {

    if dashing.is_some() {
      return;
    }

    if attacking.is_some() {
      player_movement.velocity = Vec2::ZERO;
      return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
      commands.entity(entity).insert(PlayerDash::default());
    } else if keyboard_input.just_pressed(KeyCode::F) {
        commands.entity(entity).insert(PlayerAttack::default());
    } else {
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
      player_movement.speed = player.speed;
    }
  }
}

fn player_movement(
  time: Res<Time>,
  animations: Res<PlayerAnimations>,
  mut query: Query<(&mut Velocity, &PlayerMovement), With<Player>>,
  mut query_hitbox: Query<(
    &mut Transform, 
    &mut CollisionShape,
    &mut TextureAtlasSprite,
    &mut Handle<SpriteSheetAnimation>
  ), With<PlayerHitbox>>,
) {
  if let Some((mut velocity, player_movement)) = query.iter_mut().next() {
    if let Some((
      mut hitbox_transform, 
      mut shape,
      mut sprite,
      mut animation
    )) = query_hitbox.iter_mut().next() {
      if player_movement.velocity.x > 0. {
        hitbox_transform.translation = Vec3::new(16., 0., 1.);
        sprite.flip_x = false;
        *animation = animations.attack_x.clone();
        *shape = CollisionShape::Cuboid { 
          half_extends: Vec3::new(7., 10., 1.),
          border_radius: None,
        };
      } else if player_movement.velocity.x < 0. {
        hitbox_transform.translation = Vec3::new(-16., 0., 1.);
        sprite.flip_x = true;
        *animation = animations.attack_x.clone();
        *shape = CollisionShape::Cuboid { 
          half_extends: Vec3::new(7., 10., 1.),
          border_radius: None,
        };
      } else if player_movement.velocity.y > 0. {
        hitbox_transform.translation = Vec3::new(0., 16., 1.);
        sprite.flip_y = false;
        *animation = animations.attack_y.clone();
        *shape = CollisionShape::Cuboid { 
          half_extends: Vec3::new(10., 7., 1.),
          border_radius: None,
        };
      } else if player_movement.velocity.y < 0. {
        hitbox_transform.translation = Vec3::new(0., -16., 1.);
        sprite.flip_y = true;
        *animation = animations.attack_y.clone();
        *shape = CollisionShape::Cuboid { 
          half_extends: Vec3::new(10., 7., 1.),
          border_radius: None,
        };
      }
    }

    let input_velocity = player_movement.velocity * player_movement.speed * time.delta_seconds();
    velocity.linear = Vec3::new(input_velocity.x, input_velocity.y, 1.);
  }
}

fn player_dash(
  time: Res<Time>,
  mut commands: Commands,
  mut query: Query<(Entity, &mut PlayerDash, &mut PlayerMovement), With<Player>>,
) {
  if let Some((
    entity, 
    mut player_dash, 
    mut player_movement)
  ) = query.iter_mut().next() {
    player_dash.duration.tick(time.delta());

    player_movement.speed = player_dash.speed;

    if player_dash.duration.just_finished() {
      player_movement.velocity = Vec2::ZERO;
      commands.entity(entity).remove::<PlayerDash>();
    }
  }
}

fn player_attack_added (
  mut commands: Commands,
  query_added: Query<&mut PlayerAttack, (Added<PlayerAttack>, With<Player>)>,
  mut query_player_hitbox: Query<(Entity, &mut CollisionLayers, &mut Visibility), With<PlayerHitbox>>,
) {
  if let Some(_) = query_added.iter().next() {
    for (entity, mut collision_layers, mut visibility) in query_player_hitbox.iter_mut() {
      commands.entity(entity)
        .insert(Play);
      
      visibility.is_visible = true;

      *collision_layers = collision_layers.with_group(Layer::PlayerHitbox);
    }
  }
}

fn player_attacking (
  time: Res<Time>,
  mut commands: Commands,
  mut query: Query<(Entity, &Player, &mut PlayerAttack), With<Player>>,
  mut query_player_hitbox: Query<(&mut CollisionLayers, &mut Visibility), With<PlayerHitbox>>,
) {
  for (entity, player, mut player_attack) in query.iter_mut() {
    player_attack.time_passed += time.delta_seconds();

    if player_attack.time_passed >= player.attack_duration {
      
      commands.entity(entity).remove::<PlayerAttack>();

      for (mut collision_layers, mut visibility) in query_player_hitbox.iter_mut() {
        visibility.is_visible = false;

        *collision_layers = CollisionLayers::none()
          .with_masks(&[Layer::Enemy]);
      }
    }
  }
}

fn player_pause (
  mut query: Query<(&mut Velocity, &mut PlayerMovement), With<Player>>,
) {
  if let Some((mut velocity, mut player_movement)) = query.iter_mut().next() {
    velocity.linear = Vec3::ZERO;
    player_movement.velocity = Vec2::ZERO;
  }
}

fn player_on_win (
  mut player_win_reader: EventReader<OnQuickEventPlayerWin>,
  mut query: Query<&mut Health, With<Player>>,
) {
  for _ in player_win_reader.iter() {
    if let Some(mut health) = query.iter_mut().next() {
      // maybe move the player to a safe area?
      health.fill_health();
    }
  }
}

pub fn player_despawn (
  mut commands: Commands,
  query: Query<Entity, With<Player>>,
) {
  if let Some(entity) = query.iter().next() {
    commands.entity(entity).despawn_recursive();
  }
}
