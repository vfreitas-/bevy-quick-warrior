use bevy::prelude::*;
use crate::{
  GameState, 
  character::Health, 
  player::Player, score::Score, utils::ecs::should_run
};

mod utils;
mod quick_event;
mod game_over;
pub use utils::*;
use quick_event::*;
use game_over::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum UiLabel {
  Setup,
  HUD,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(GameState::Starting)
          .with_system(ui_setup.label(UiLabel::Setup))
          // TODO: use this instead of a new set on the running state
          // this is not working idk why
          // .with_system(
          //   ui_player_spawn
          //     .label(UiLabel::HUD)
          //     .after(UiLabel::Setup)
          // )
      )
      .add_system_set(
        SystemSet::on_enter(GameState::Running)
          .with_run_criteria(should_run::<UIPlayerHUD>)
          .with_system(ui_player_spawn)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(ui_health_bar)
          .with_system(score_update)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::TimedEvent)
          .with_system(ui_quick_event_spawn)
      )
      .add_system_set(
        SystemSet::on_update(GameState::TimedEvent)
          .with_system(ui_update_event_count)
          .with_system(quick_event_countdown)
          .with_system(ui_quick_event_results)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::TimedEvent)
          .with_system(ui_quick_event_despawn)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::GameOver)
          .with_system(hud_despawn)
          .with_system(ui_game_over_spawn)
      )
      .add_system_set(
        SystemSet::on_update(GameState::GameOver)
          .with_system(game_over_input)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::GameOver)
          .with_system(game_over_despawn)
      );
  }
}

#[derive(Component)]
pub struct UIRootNode;

#[derive(Component)]
pub struct UIPlayerHUD;

#[derive(Component)]
pub struct UIPlayerHealthbar;

#[derive(Component)]
pub struct UIPlayerHealthbarLife;

#[derive(Component)]
pub struct UIHUDScoreText;

fn ui_setup (
  mut commands: Commands,
) {
  commands.spawn_bundle(
    UiCameraBundle::default()
  );

  commands.spawn_bundle(
    NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        flex_direction: FlexDirection::ColumnReverse,
        ..Default::default()
      },
      color: Color::NONE.into(),
      ..Default::default()
    }
  )
  .insert(UIRootNode);
}

fn ui_player_spawn (
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut query: Query<Entity, With<UIRootNode>>,
) {
  if let Some(root) = query.iter_mut().next() {
    commands.entity(root)
      .with_children(|parent| {

        parent.spawn_bundle(
          NodeBundle {
            style: Style {
              size: Size::new(Val::Percent(100.), Val::Px(100.)),
              justify_content: JustifyContent::SpaceBetween,
              align_items: AlignItems::Center,
              ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
          }
        )
        .insert(UIPlayerHUD)
        .with_children(|player_hud| {

          player_hud.spawn_bundle(
            NodeBundle {
              style: Style {
                padding: Rect::all(Val::Px(24.)),
                size: Size::new(Val::Percent(100.), Val::Px(80.)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
              },
              color: Color::NONE.into(),
              ..Default::default()
            }
          )
          .insert(UIPlayerHealthbar);

          // Score text
          player_hud.spawn_bundle(
            NodeBundle {
              style: Style {
                padding: Rect::all(Val::Px(24.)),
                ..Default::default()
              },
              color: Color::NONE.into(),
              ..Default::default()
            }
          )
          .with_children(|parent| {
            parent.spawn_bundle(
              TextBundle {
                text: Text {
                  sections: vec![
                    TextSection {
                      value: "Score: ".to_string(),
                      style: TextStyle {
                        font: asset_server.load("Fonts/KenneyPixel.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE, 
                      },
                    },
                    TextSection {
                      value: "0000".to_string(),
                      style: TextStyle {
                        font: asset_server.load("Fonts/KenneyPixel.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE, 
                      },
                    }
                  ],
                  alignment: Default::default()
                },
                ..Default::default()
              }
            )
            .insert(UIHUDScoreText);
          });

        });
      });
  }
}

fn ui_health_bar (
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut query_ui: Query<Entity, With<UIPlayerHealthbar>>,
  mut query_health_child: Query<&mut Visibility, With<UIPlayerHealthbarLife>>,
  query_health: Query<&Health, (Changed<Health>, With<Player>)>,
) {
  for player_health in query_health.iter() {
    for entity in query_ui.iter_mut() {
      if !query_health_child.is_empty() {
        for (index, mut visibility) in query_health_child.iter_mut().enumerate() {
          visibility.is_visible = if (index + 1) <= player_health.health {
            true
          } else {
            false
          };
        }
      } else {
        commands.entity(entity)
          .with_children(|parent| {
            for index in 0..player_health.max_health {
              parent.spawn_bundle(
                ImageBundle {
                  image: UiImage(asset_server.load("Art/UI/Health.png")),
                  style: Style {
                    size: Size::new(Val::Px(40.), Val::Px(40.)),
                    margin: Rect {
                      right: Val::Px(8.),
                      ..Default::default()
                    },
                    ..Default::default()
                  },
                  visibility: Visibility {
                    is_visible: if (index + 1) <= player_health.health { true } else { false },
                  },
                  ..Default::default()
                },
              )
              .insert(UIPlayerHealthbarLife);
            }
          }); 
      }

    }
  }
}

pub fn score_update (
  score: Res<Score>,
  mut query: Query<&mut Text, With<UIHUDScoreText>>,
) {
  if let Some(mut text) = query.iter_mut().next() {
    text.sections[1].value = format!(
      "{:04}",
      score.points
    );
  }
}

pub fn hud_despawn (
  mut commands: Commands,
  query: Query<Entity, With<UIRootNode>>,
) {
  if let Some(entity) = query.iter().next() {
    commands.entity(entity).despawn_recursive();
  }
}
