use bevy::{prelude::*, ecs::schedule::ShouldRun};

#[allow(dead_code)]
pub fn should_run<T:Component> (
  query: Query<Entity, With<T>>,
) -> ShouldRun {
  if let Some(_) = query.iter().next() {
    return ShouldRun::No;
  } else {
    return ShouldRun::Yes;
  }
}
