use bevy::prelude::*;
use crate::{GameState, quick_event::OnQuickEvent, utils::ecs::should_run};

mod quick_event;
use quick_event::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(ui_setup)
      .add_system_set(
        SystemSet::on_enter(GameState::Running)
          .with_run_criteria(should_run::<UIPlayerHUD>)
          .with_system(ui_player_spawn)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Running)
          .with_system(button_system)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::TimedEvent)
          .with_system(ui_quick_event_spawn)
      )
      .add_system_set(
        SystemSet::on_update(GameState::TimedEvent)
          .with_system(ui_update_event_count)
      )
      .add_system_set(
        SystemSet::on_exit(GameState::TimedEvent)
          .with_system(ui_quick_event_despawn)
      );
  }
}

#[derive(Component)]
pub struct UIRootNode;

#[derive(Component)]
pub struct UIPlayerHUD;

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
            ButtonBundle {
              style: Style {
                padding: Rect::all(Val::Px(8.)),
                size: Size::new(Val::Auto, Val::Px(60.)),
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
                  font_size: 40.0,
                  color: Color::WHITE,
                },
                Default::default()
              ),
              ..Default::default()
            });
          });
        });
      });
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
