use bevy::prelude::*;
use benimator::*;

#[cfg(all(feature = "debug"))]
use bevy_inspector_egui::WorldInspectorPlugin;

mod utils;
mod camera;
mod physics;
mod player;
mod enemy;
mod character;
mod level;
mod quick_event;
mod ui;
mod score;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
  MainMenu,
  Starting,
  Running,
  TimedEvent,
  GameOver,
}

fn main() {
  let mut app = App::new();

  app
    .insert_resource(WindowDescriptor {
      title: "Quick Warrior".to_string(),
      width: 960.0, // 1280 - 960 - 640
      height: 720.0, // 960 - 720 - 480
      resizable: false,
      vsync: true,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_plugin(AnimationPlugin::default())
    .add_state(GameState::MainMenu)
    .add_plugin(physics::AppPhysicsPlugin)
    .add_plugin(level::LevelPlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(character::CharacterPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(enemy::EnemyPlugin)
    .add_plugin(quick_event::QuickEventPlugin)
    .add_plugin(ui::UIPlugin)
    .add_plugin(score::ScorePlugin)
    .add_system(bevy::input::system::exit_on_esc_system);

  #[cfg(all(feature = "debug"))]
  app.add_plugin(WorldInspectorPlugin::new());

  app.run();
}
