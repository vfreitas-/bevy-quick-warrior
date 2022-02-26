use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(map_startup);
  }
}

fn map_startup (
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
