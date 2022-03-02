use bevy::prelude::*;
use crate::{
  GameState, 
  quick_event::OnQuickEvent,
  character::Health, 
  player::Player
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
          .with_system(ui_player_spawn)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(ui_health_bar)
          .with_system(button_system)
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
pub struct UIEventBtn;

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
              align_self: AlignSelf::FlexEnd,
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
          }
        )
        .insert(UIPlayerHUD)
        .with_children(|parent| {

          parent.spawn_bundle(
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
          
          // Quick Event Button
          #[cfg(all(feature = "debug"))]
          parent.spawn_bundle(
            ButtonBundle {
              style: Style {
                position_type: PositionType::Absolute,
                padding: Rect::all(Val::Px(8.)),
                size: Size::new(Val::Auto, Val::Px(40.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
              },
              color: Color::DARK_GRAY.into(),
              ..Default::default()
            }
          )
          .insert(UIEventBtn)
          .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
              text: Text::with_section(
                "Quick Event",
                TextStyle {
                  font: asset_server.load("Fonts/KenneyPixel.ttf"),
                  font_size: 24.0,
                  color: Color::WHITE,
                },
                Default::default()
              ),
              ..Default::default()
            });
          }); // Quick Event Button


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

fn button_system(
  mut interaction_query: Query<
    &Interaction,
    (Changed<Interaction>, With<UIEventBtn>),
  >,
  mut event_writer: EventWriter<OnQuickEvent>,
) {
  for interaction in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        event_writer.send(OnQuickEvent);
      }
      _ => (),
    }
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
