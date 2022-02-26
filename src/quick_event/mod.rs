use bevy::{prelude::*, core::FixedTimestep};

use crate::GameState;

pub struct QuickEventPlugin;

impl Plugin for QuickEventPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<OnQuickEvent>()
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(quick_event_listener)
      )
      .add_system_set(
        SystemSet::on_update(GameState::TimedEvent)
          .with_system(quick_event_input)
      );
  }
}

pub struct OnQuickEvent;

fn quick_event_listener (
  mut events: EventReader<OnQuickEvent>,
  mut state: ResMut<State<GameState>>,
) {
  for _ in events.iter() {
    if &GameState::Running == state.current() {
      state.set(GameState::TimedEvent).unwrap();
    }
  }
}

fn quick_event_input (
  keyboard_input: Res<Input<KeyCode>>,
  mut commands: Commands,
) {

}
