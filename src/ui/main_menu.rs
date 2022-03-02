use bevy::prelude::*;

use crate::{GameState};

use super::basic_text;

#[derive(Component, Debug)]
pub struct UIMainMenuRoot;

pub fn ui_main_menu_spawn(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
) {

  commands.spawn_bundle(
    NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        flex_direction: FlexDirection::ColumnReverse,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      color: Color::rgb(0., 0., 0.).into(),
      ..Default::default()
    }
  )
  .insert(UIMainMenuRoot)
  .with_children(|root| {

    let font = asset_server.load("Fonts/KenneyPixel.ttf");

    root.spawn_bundle(
      basic_text("Quick Warrior", 96.0, font.clone(), Some(64.)),
    );

    root.spawn_bundle(
      basic_text("Press space to start the game", 24., font.clone(), None),
    );

  });

}

pub fn main_menu_input (
  keyboard_input: Res<Input<KeyCode>>,
  mut state: ResMut<State<GameState>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    if &GameState::MainMenu == state.current() {
      state.set(GameState::Starting).unwrap();
    }
  }
}

pub fn main_menu_despawn (
  mut commands: Commands,
  query: Query<Entity, With<UIMainMenuRoot>>,
) {
  if let Some(entity) = query.iter().next() {
    commands.entity(entity).despawn_recursive();
  }
}
