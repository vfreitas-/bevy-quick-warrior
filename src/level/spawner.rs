use bevy::prelude::*;
use rand::prelude::*;
use crate::enemy::*;


#[derive(Debug, PartialEq)]
pub struct SpawnData {
  wave: i32,
  max_enemies: i32,
}

impl Default for SpawnData {
  fn default() -> SpawnData {
    SpawnData {
      wave: 0,
      max_enemies: 3,
    }
  }
}

pub fn spawn_enemies(
  spawn_data: Res<SpawnData>,
  mut commands: Commands,
) {
  let mut rng = rand::thread_rng();
  let max = rng.gen_range(0..spawn_data.max_enemies);

  for n in 0..max {
    let x = rng.gen_range(-320.0..320.0);
    let y = rng.gen_range(-240.0..240.0);

    enemy_spawn(&mut commands, Transform {
      translation: Vec3::new(x, y, 1.),
      ..Default::default()
    });
  }
}
