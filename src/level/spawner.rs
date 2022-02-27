use bevy::prelude::*;
use rand::prelude::*;
use crate::enemy::*;

#[derive(Debug, PartialEq)]
pub struct SpawnData {
  wave: i32,
  min_enemies: usize,
  max_enemies: usize,
  time_passed: f32,
  interval: f32,
  enemy_count: usize,
}

impl Default for SpawnData {
  fn default() -> SpawnData {
    SpawnData {
      wave: 0,
      min_enemies: 1,
      max_enemies: 3,
      time_passed: 0.,
      interval: 6.,
      enemy_count: 0,
    }
  }
}

pub fn spawn_enemies_constant(
  time: Res<Time>,
  mut spawn_data: ResMut<SpawnData>,
  mut commands: Commands,
) {
  spawn_data.time_passed += time.delta_seconds();

  if spawn_data.time_passed < spawn_data.interval {
    return;
  }

  let mut rng = rand::thread_rng();
  let max = rng.gen_range(spawn_data.min_enemies..spawn_data.max_enemies);

  for _ in 0..max {
    let x = rng.gen_range(-320.0..320.0);
    let y = rng.gen_range(-240.0..240.0);

    enemy_spawn(&mut commands, Transform {
      translation: Vec3::new(x, y, 1.),
      ..Default::default()
    });

    spawn_data.enemy_count += 1;
  }

  spawn_data.time_passed = 0.;
  spawn_data.wave += 1;
  spawn_data.min_enemies = (spawn_data.wave / 4) as usize + 1;
  spawn_data.max_enemies = (spawn_data.wave / 2) as usize + 3;
}
