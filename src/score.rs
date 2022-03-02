use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<Score>()
      .add_event::<OnScorePoints>()
      .add_system_set(
        SystemSet::on_enter(GameState::Starting)
          .with_system(score_startup)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(score_events)
      )
      .add_system_set(
        SystemSet::on_update(GameState::TimedEvent)
          .with_system(score_events)
      );
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScoreTypes {
  Enemy,
  Duel,
}

#[derive(Debug, Clone)]
pub struct OnScorePoints(pub ScoreTypes);

#[derive(Debug, Clone)]
pub struct Score {
  pub points: usize,
}

impl Default for Score {
  fn default () -> Self {
    Self {
      points: 0,
    }
  }
}

fn score_startup (
  mut commands: Commands,
) {
  commands.insert_resource(Score::default());
}

fn score_events (
  mut score: ResMut<Score>,
  mut event_reader: EventReader<OnScorePoints>,
) {

  for event in event_reader.iter() {
    match event.0 {
      ScoreTypes::Enemy => score.points += 100,
      ScoreTypes::Duel => score.points += 500,
    }
  }

}
