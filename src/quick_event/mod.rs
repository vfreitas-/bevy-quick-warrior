use bevy::prelude::*;
use rand::prelude::*;

use crate::GameState;

#[allow(dead_code)]
const KEYBINDS: [[KeyCode; 4]; 9] = [ 
  [KeyCode::Key1, KeyCode::Key0, KeyCode::O, KeyCode::K],
  [KeyCode::Key2, KeyCode::Q, KeyCode::P, KeyCode::L],
  [KeyCode::Key3, KeyCode::W, KeyCode::A, KeyCode::Z],
  [KeyCode::Key4, KeyCode::E, KeyCode::S, KeyCode::X],
  [KeyCode::Key5, KeyCode::R, KeyCode::D, KeyCode::C],
  [KeyCode::Key6, KeyCode::T, KeyCode::F, KeyCode::V],
  [KeyCode::Key7, KeyCode::Y, KeyCode::G, KeyCode::B],
  [KeyCode::Key8, KeyCode::U, KeyCode::H, KeyCode::N],
  [KeyCode::Key9, KeyCode::I, KeyCode::J, KeyCode::M],
];

pub struct QuickEventPlugin;

impl Plugin for QuickEventPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<QuickEventData>()
      .add_event::<OnQuickEvent>()
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(quick_event_listener)
      )
      .add_system_set(
        SystemSet::on_update(GameState::TimedEvent)
          .with_system(quick_event_input)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::TimedEvent)
          .with_system(quick_event_on_exit)
      );
  }
}

pub struct OnQuickEvent;

#[derive(Debug, Clone)]
pub struct QuickEventData {
  pub index: IVec2,
  pub key: KeyCode,
  pub count: usize,
}

impl Default for QuickEventData {
    fn default () -> Self {
      Self {
        index: IVec2::ZERO,
        key: KeyCode::A,
        count: 0,
      }
    }
}

fn quick_event_listener (
  mut quick_event_data: ResMut<QuickEventData>,
  mut events: EventReader<OnQuickEvent>,
  mut state: ResMut<State<GameState>>,
) {
  for _ in events.iter() {
    if &GameState::Running == state.current() {
      let mut rng = rand::thread_rng();
      let x = rng.gen_range(0..8);
      let y = rng.gen_range(0..3);
      quick_event_data.index = IVec2::new(x, y);
      quick_event_data.key = KEYBINDS[x as usize][y as usize];

      state.set(GameState::TimedEvent).unwrap();
    }
  }
}

fn quick_event_input (
  keyboard_input: Res<Input<KeyCode>>,
  mut quick_event_data: ResMut<QuickEventData>,
  mut state: ResMut<State<GameState>>,
  mut commands: Commands,
) {

  if keyboard_input.just_pressed(quick_event_data.key) {
    quick_event_data.count += 1;
  }

  if keyboard_input.just_pressed(KeyCode::Tab) {
    if &GameState::TimedEvent == state.current() {
      state.set(GameState::Running).unwrap();
    }
  }

}

fn quick_event_on_exit (
  mut commands: Commands,
) {
  commands.insert_resource(QuickEventData::default());
}
