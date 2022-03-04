use bevy::prelude::*;

use crate::{GameState, score::Score};

use super::basic_text;

#[derive(Component, Debug)]
pub struct UIGameOverRoot;

pub fn ui_game_over_spawn(
  asset_server: Res<AssetServer>,
  score: Res<Score>,
  mut commands: Commands,
) {

  commands.spawn_bundle(
    NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
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
  .insert(UIGameOverRoot)
  .with_children(|root| {

    let font = asset_server.load("Fonts/KenneyPixel.ttf");

    root.spawn_bundle(
      basic_text("Game Over!", 96.0, font.clone(), Some(48.)),
    );

    root.spawn_bundle(
      basic_text(
        format!("You made: {:04} points", score.points).as_str(), 
        40.0, 
        font.clone(), 
        Some(16.)
      ),
    );

    let text = if score.points <= 3000 {
      "You were very slow"
    } else if score.points <= 15000 {
      "You did well, but could be quicker"
    } else {
      "You are quicker as a bullet. Congrats!"
    };

    root.spawn_bundle(
      basic_text(text, 40., font.clone(), Some(48.))
    );

    root.spawn_bundle(
      basic_text("Press space to try again.", 24., font.clone(), None),
    );

  });

}

pub fn game_over_input (
  keyboard_input: Res<Input<KeyCode>>,
  mut state: ResMut<State<GameState>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    if &GameState::GameOver == state.current() {
      state.set(GameState::Starting).unwrap();
    }
  }
}

pub fn game_over_despawn (
  mut commands: Commands,
  query: Query<Entity, With<UIGameOverRoot>>,
) {
  if let Some(entity) = query.iter().next() {
    commands.entity(entity).despawn_recursive();
  }
}
