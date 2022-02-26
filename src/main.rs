use bevy::prelude::*;

#[cfg(all(feature = "debug"))]
use bevy_inspector_egui::WorldInspectorPlugin;

mod utils;
mod camera;
mod physics;
mod player;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
  MainMenu,
  Running,
  TimedEvent,
  GameOver,
}

fn main() {
  let mut app = App::new();

  app
    .insert_resource(WindowDescriptor {
      title: "Template".to_string(),
      width: 960.0, // 1280 - 960 - 640
      height: 720.0, // 960 - 720 - 480
      resizable: false,
      vsync: true,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_state(GameState::Running)
    .add_plugin(physics::AppPhysicsPlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_system(bevy::input::system::exit_on_esc_system);

  #[cfg(all(feature = "debug"))]
  app.add_plugin(WorldInspectorPlugin::new());

  app.run();
}
