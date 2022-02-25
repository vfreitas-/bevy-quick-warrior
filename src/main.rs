use bevy::prelude::*;

#[cfg(all(feature = "debug"))]
use bevy_inspector_egui::WorldInspectorPlugin;

mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
  Playing,
}

fn main() {
  let mut app = App::new();

  app
    .insert_resource(WindowDescriptor {
      title: "Template".to_string(),
      width: 640.0,
      height: 480.0,
      vsync: true,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_state(GameState::Playing)
    .add_system(bevy::input::system::exit_on_esc_system);

  #[cfg(all(feature = "debug"))]
  app.add_plugin(WorldInspectorPlugin::new());

  app.run();
}
