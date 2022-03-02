use bevy::prelude::*;
use rand::prelude::*;

use crate::{GameState, score::{ScoreTypes, OnScorePoints}};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum QuickEventState {
  Start,
  Running,
  Results,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum QuickEventWinner {
  Player,
  Enemies,
}

#[derive(Debug, Clone)]
pub struct KeyBind {
  pub key: KeyCode,
  pub sprite: &'static str,
  pub label: &'static str,
}

#[allow(dead_code)]
const KEYBINDS: [KeyBind; 36] = [
  KeyBind { key: KeyCode::Key0, sprite: "Art/UI/Keybinds/00.png", label: "0" },
  KeyBind { key: KeyCode::Key1, sprite: "Art/UI/Keybinds/01.png", label: "1" },
  KeyBind { key: KeyCode::Key2, sprite: "Art/UI/Keybinds/02.png", label: "2" },
  KeyBind { key: KeyCode::Key3, sprite: "Art/UI/Keybinds/03.png", label: "3" },
  KeyBind { key: KeyCode::Key4, sprite: "Art/UI/Keybinds/04.png", label: "4" },
  KeyBind { key: KeyCode::Key5, sprite: "Art/UI/Keybinds/05.png", label: "5" },
  KeyBind { key: KeyCode::Key6, sprite: "Art/UI/Keybinds/06.png", label: "6" },
  KeyBind { key: KeyCode::Key7, sprite: "Art/UI/Keybinds/07.png", label: "7" },
  KeyBind { key: KeyCode::Key8, sprite: "Art/UI/Keybinds/08.png", label: "8" },
  KeyBind { key: KeyCode::Key9, sprite: "Art/UI/Keybinds/09.png", label: "9" },
  KeyBind { key: KeyCode::A, sprite: "Art/UI/Keybinds/A.png", label: "A" },
  KeyBind { key: KeyCode::B, sprite: "Art/UI/Keybinds/B.png", label: "B" },
  KeyBind { key: KeyCode::C, sprite: "Art/UI/Keybinds/C.png", label: "C" },
  KeyBind { key: KeyCode::D, sprite: "Art/UI/Keybinds/D.png", label: "D" },
  KeyBind { key: KeyCode::E, sprite: "Art/UI/Keybinds/E.png", label: "E" },
  KeyBind { key: KeyCode::F, sprite: "Art/UI/Keybinds/F.png", label: "F" },
  KeyBind { key: KeyCode::G, sprite: "Art/UI/Keybinds/G.png", label: "G" },
  KeyBind { key: KeyCode::H, sprite: "Art/UI/Keybinds/H.png", label: "H" },
  KeyBind { key: KeyCode::I, sprite: "Art/UI/Keybinds/I.png", label: "I" },
  KeyBind { key: KeyCode::J, sprite: "Art/UI/Keybinds/J.png", label: "J" },
  KeyBind { key: KeyCode::K, sprite: "Art/UI/Keybinds/K.png", label: "K" },
  KeyBind { key: KeyCode::L, sprite: "Art/UI/Keybinds/L.png", label: "L" },
  KeyBind { key: KeyCode::M, sprite: "Art/UI/Keybinds/M.png", label: "M" },
  KeyBind { key: KeyCode::N, sprite: "Art/UI/Keybinds/N.png", label: "N" },
  KeyBind { key: KeyCode::O, sprite: "Art/UI/Keybinds/O.png", label: "O" },
  KeyBind { key: KeyCode::P, sprite: "Art/UI/Keybinds/P.png", label: "P" },
  KeyBind { key: KeyCode::Q, sprite: "Art/UI/Keybinds/Q.png", label: "Q" },
  KeyBind { key: KeyCode::R, sprite: "Art/UI/Keybinds/R.png", label: "R" },
  KeyBind { key: KeyCode::S, sprite: "Art/UI/Keybinds/S.png", label: "S" },
  KeyBind { key: KeyCode::T, sprite: "Art/UI/Keybinds/T.png", label: "T" },
  KeyBind { key: KeyCode::U, sprite: "Art/UI/Keybinds/U.png", label: "U" },
  KeyBind { key: KeyCode::V, sprite: "Art/UI/Keybinds/V.png", label: "V" },
  KeyBind { key: KeyCode::W, sprite: "Art/UI/Keybinds/W.png", label: "W" },
  KeyBind { key: KeyCode::X, sprite: "Art/UI/Keybinds/X.png", label: "X" },
  KeyBind { key: KeyCode::Y, sprite: "Art/UI/Keybinds/Y.png", label: "Y" },
  KeyBind { key: KeyCode::Z, sprite: "Art/UI/Keybinds/Z.png", label: "Z" },
];

pub struct OnQuickEvent;

pub struct OnQuickEventEnd;

pub struct OnQuickEventPlayerWin;

pub struct QuickEventPlugin;
impl Plugin for QuickEventPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<QuickEvent>()
      .init_resource::<QuickEventData>()
      .add_event::<OnQuickEvent>()
      .add_event::<OnQuickEventEnd>()
      .add_event::<OnQuickEventPlayerWin>()
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(quick_event_listener)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::TimedEvent)
          .with_system(quick_event_startup)
      )
      .add_system_set(
        SystemSet::on_update(GameState::TimedEvent)
          .with_system(quick_event_input)
          .with_system(quick_event_time_track)
          .with_system(quick_event_on_end)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::TimedEvent)
          .with_system(quick_event_on_exit)
      );
  }
}

#[derive(Debug, Clone)]
pub struct QuickEvent {
  pub state: QuickEventState,
  duration: f32,
  // Keys pressed per second
  enemy_speed: f32,
}

impl Default for QuickEvent {
  fn default () -> Self {
    Self {
      state: QuickEventState::Start,
      duration: 5.,
      enemy_speed: 1.,
    }
  }
}

#[derive(Debug, Clone)]
pub struct QuickEventData {
  pub keybind: KeyBind,
  pub player_count: usize,
  pub enemy_count: usize,
  pub time_passed: f32,
  pub winner: Option<QuickEventWinner>,
}

impl Default for QuickEventData {
  fn default () -> Self {
    Self {
      keybind: KEYBINDS[0].clone(),
      player_count: 0,
      enemy_count: 0,
      time_passed: 0.0,
      winner: None,
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
      let index = rng.gen_range(0..35);

      quick_event_data.keybind = KEYBINDS[index as usize].clone();

      if let Err(changed) = state.set(GameState::TimedEvent) {
        println!("[QuickEvent] {:?}", changed);
      }
    }
  }
}

fn quick_event_startup () {}

fn quick_event_input (
  keyboard_input: Res<Input<KeyCode>>,
  quick_event: Res<QuickEvent>,
  mut quick_event_data: ResMut<QuickEventData>,
  mut state: ResMut<State<GameState>>,
) {
  if quick_event.state != QuickEventState::Running {
    return;
  }

  let enemy_interval = quick_event.enemy_speed / quick_event.duration;
  quick_event_data.enemy_count = (quick_event_data.time_passed / enemy_interval).floor() as usize;

  if keyboard_input.just_pressed(quick_event_data.keybind.key) {
    quick_event_data.player_count += 1;
  }

  if keyboard_input.just_pressed(KeyCode::Tab) {
    if &GameState::TimedEvent == state.current() {
      state.set(GameState::Running).unwrap();
    }
  }
}

fn quick_event_time_track (
  time: Res<Time>,
  mut quick_event: ResMut<QuickEvent>,
  mut quick_event_data: ResMut<QuickEventData>,
) {
  if quick_event.state != QuickEventState::Running {
    return;
  }

  quick_event_data.time_passed += time.delta_seconds();

  if quick_event_data.time_passed >=quick_event.duration {
    // End Duel!
    quick_event_data.winner = Some(
      // The enemies have the draw advantage here :eyes:
      // maybe i should put it in the ui somehow?
      if quick_event_data.player_count > quick_event_data.enemy_count {
        QuickEventWinner::Player
      } else {
        QuickEventWinner::Enemies
      }
    );
    quick_event.state = QuickEventState::Results;
  }
}

fn quick_event_on_end (
  quick_event_data: Res<QuickEventData>,
  mut quick_event: ResMut<QuickEvent>,
  mut event_reader: EventReader<OnQuickEventEnd>,
  mut player_win_writer: EventWriter<OnQuickEventPlayerWin>,
  mut score_writer: EventWriter<OnScorePoints>,
  mut state: ResMut<State<GameState>>,
) {
  for _ in event_reader.iter() {
    if &GameState::TimedEvent == state.current() {
      quick_event.state = QuickEventState::Start;
      if quick_event_data.winner == Some(QuickEventWinner::Player) {
        // return to the game and send an event to the player and level
        state.set(GameState::Running).unwrap();
        player_win_writer.send(OnQuickEventPlayerWin);
        score_writer.send(OnScorePoints(ScoreTypes::Duel));
        
      } else {
        if let Err(changed) = state.set(GameState::GameOver) {
          println!("[QuickEvent] {:?}", changed);
        }
      }
    }
  }
}

fn quick_event_on_exit (
  mut commands: Commands,
) {
  commands.insert_resource(QuickEventData::default());
}
