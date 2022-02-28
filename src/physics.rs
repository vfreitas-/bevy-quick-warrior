use bevy::prelude::*;
use heron::prelude::*;

#[allow(dead_code)]
#[derive(PhysicsLayer)]
pub enum Layer {
  World,
  Player,
  PlayerHitbox,
  PlayerHurtbox,
  EnemyHurtbox,
  Enemy,
  SoftCollisions,
}

pub struct AppPhysicsPlugin;

impl Plugin for AppPhysicsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(PhysicsPlugin::default())
      .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0)))
      .register_type::<CollisionLayers>();
  }
}
