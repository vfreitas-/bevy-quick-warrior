use bevy::prelude::*;

use crate::quick_event::*;

use super::UIRootNode;

#[derive(Component)]
pub struct UIQuickEventPopup;

#[derive(Component)]
pub struct UIQuickEventPlayerKeySprite;

#[derive(Component)]
pub struct UIQuickEventPlayerCount;

#[derive(Component)]
pub struct UIQuickEventEnemyCount;

#[derive(Component)]
pub struct UIQuickEventCountdown {
  duration: Timer,
  count: usize,
}

impl Default for UIQuickEventCountdown {
  fn default () -> Self {
    Self {
      duration: Timer::from_seconds(1., true),
      count: 2,
    }
  }
}

#[derive(Component)]
pub struct UIQuickEventResults {
  duration: Timer,
}

impl Default for UIQuickEventResults {
  fn default () -> Self {
    Self {
      duration: Timer::from_seconds(3.5, false),
    }
  }
}

pub fn ui_quick_event_spawn (
  asset_server: Res<AssetServer>,
  quick_event_data: Res<QuickEventData>,
  mut commands: Commands,
  mut query: Query<Entity, With<UIRootNode>>,
) {

  if let Some(root) = query.iter_mut().next() {
    commands.entity(root)
      .with_children(|parent| {
        parent.spawn_bundle(
          NodeBundle {
            style: Style {
              position_type: PositionType::Absolute,
              size: Size::new(Val::Percent(100.), Val::Percent(100.)),
              align_items: AlignItems::Center,
              justify_content: JustifyContent::Center,
              ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.75).into(),
            ..Default::default()
          }
        ) // Absolute overlay
        .insert(UIQuickEventPopup)
        .with_children(|parent| {
          parent.spawn_bundle(
            NodeBundle {
              style: Style {
                size: Size::new(Val::Percent(60.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
              },
              color: Color::NONE.into(),
              ..Default::default()
            }
          )
          .with_children(|parent| {
            
            // Header
            parent.spawn_bundle(
              NodeBundle {
                style: Style {
                  size: Size::new(Val::Percent(100.), Val::Auto),
                  flex_direction: FlexDirection::ColumnReverse,
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  margin: Rect {
                    bottom: Val::Px(32.),
                    ..Default::default()
                  },
                  ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
              }
            )
            .with_children(|parent| {

              // Title
              parent.spawn_bundle(
                TextBundle {
                  style: Style {
                    margin: Rect {
                      bottom: Val::Px(16.),
                      ..Default::default()
                    },
                    ..Default::default()
                  },
                  text: Text::with_section(
                    "Quick Duel",
                    TextStyle {
                      font: asset_server.load("Fonts/KenneyPixel.ttf"),
                      font_size: 60.0,
                      color: Color::WHITE, 
                    },
                    Default::default()
                  ),
                  ..Default::default()
                }
              );

              // Decription
              parent.spawn_bundle(
                TextBundle {
                  style: Style {
                    max_size: Size::new(Val::Px(400.), Val::Auto),
                    ..Default::default()
                  },
                  text: Text::with_section(
                    "Press the random keys fastest as you can to win the duel!",
                    TextStyle {
                      font: asset_server.load("Fonts/KenneyPixel.ttf"),
                      font_size: 24.0,
                      color: Color::WHITE, 
                    },
                    Default::default()
                  ),
                  ..Default::default()
                }
              );
            });

            // Wrapper
            parent.spawn_bundle(
              NodeBundle {
                style: Style {
                  margin: Rect {
                    bottom: Val::Px(16.),
                    ..Default::default()
                  },
                  size: Size::new(Val::Percent(100.), Val::Auto),
                  justify_content: JustifyContent::SpaceBetween,
                  align_items: AlignItems::Center,
                  ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
              }
            )
            .with_children(|parent| {

              // Player Column
              parent.spawn_bundle(
                NodeBundle {
                  style: Style {
                    padding: Rect::all(Val::Px(8.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..Default::default()
                  },
                  color: Color::NONE.into(),
                  ..Default::default()
                }
              )
              .with_children(|parent| {

                // Column Name
                parent.spawn_bundle(
                  TextBundle {
                    style: Style {
                      margin: Rect {
                        bottom: Val::Px(24.),
                        ..Default::default()
                      },
                      ..Default::default()
                    },
                    text: Text::with_section(
                      "Player",
                      TextStyle {
                        font: asset_server.load("Fonts/KenneyPixel.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE, 
                      },
                      Default::default()
                    ),
                    ..Default::default()
                  }
                );

                parent.spawn_bundle(
                  NodeBundle {
                    style: Style {
                      justify_content: JustifyContent::Center,
                      align_items: AlignItems::Center,
                      ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                  }
                )
                .with_children(|parent| {
                  let keybinds = quick_event_data.keybinds.clone();
                  for keybind in keybinds {
                    parent.spawn_bundle(
                      ImageBundle {
                        image: UiImage(asset_server.load(keybind.sprite)),
                        style: Style {
                          size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                          margin: Rect {
                            right: Val::Px(4.),
                            bottom: Val::Px(24.),
                            ..Default::default()
                          },
                          ..Default::default()
                        },
                        ..Default::default()
                      }
                    )
                    .insert(UIQuickEventPlayerKeySprite);
                  }
                });


                parent.spawn_bundle(
                  TextBundle {
                    text: Text::with_section(
                      "00",
                      TextStyle {
                        font: asset_server.load("Fonts/KenneyPixel.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE, 
                      },
                      Default::default()
                    ),
                    ..Default::default()
                  }
                )
                .insert(UIQuickEventPlayerCount);

              });

              // Enemy Column
              parent.spawn_bundle(
                NodeBundle {
                  style: Style {
                    padding: Rect::all(Val::Px(8.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..Default::default()
                  },
                  color: Color::NONE.into(),
                  ..Default::default()
                }
              )
              .with_children(|parent| {

                // Text
                parent.spawn_bundle(
                  TextBundle {
                    style: Style {
                      margin: Rect {
                        bottom: Val::Px(24.),
                        ..Default::default()
                      },
                      ..Default::default()
                    },
                    text: Text::with_section(
                      "Enemy",
                      TextStyle {
                        font: asset_server.load("Fonts/KenneyPixel.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE, 
                      },
                      Default::default()
                    ),
                    ..Default::default()
                  }
                );

                parent.spawn_bundle(
                  TextBundle {
                    text: Text::with_section(
                      "00",
                      TextStyle {
                        font: asset_server.load("Fonts/KenneyPixel.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE, 
                      },
                      Default::default()
                    ),
                    ..Default::default()
                  }
                )
                .insert(UIQuickEventEnemyCount);

              });
            });

            // Timer
            parent.spawn_bundle(
              NodeBundle {
                style: Style {
                  position_type: PositionType::Absolute,
                  size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                  align_items: AlignItems::Center,
                  justify_content: JustifyContent::Center,
                  ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
              }
            )
            .with_children(|timer| {

              // Text
              timer.spawn_bundle(
                TextBundle {
                  text: Text::with_section(
                    "2",
                    TextStyle {
                      font: asset_server.load("Fonts/KenneyPixel.ttf"),
                      font_size: 100.0,
                      color: Color::hex("e89bac").unwrap(), 
                    },
                    Default::default()
                  ),
                  ..Default::default()
                }
              )
              .insert(UIQuickEventCountdown::default());

            });

          });
        });
      }); // 
  }
}

pub fn quick_event_countdown (
  time: Res<Time>,
  mut commands: Commands,
  mut quick_event: ResMut<QuickEvent>,
  mut query: Query<(Entity, &mut Text, &mut UIQuickEventCountdown)>,
) {
  if quick_event.state != QuickEventState::Start {
    return;
  }

  for (entity, mut text, mut countdown) in query.iter_mut() {
    countdown.duration.tick(time.delta());

    if countdown.duration.just_finished() {
      if countdown.count < 1 {

        countdown.duration.set_repeating(false);
        countdown.duration.reset();
        countdown.duration.pause();
        quick_event.state = QuickEventState::Running;
        commands.entity(entity).despawn_recursive();
        return;
      }
      
      countdown.count -= 1;

      text.sections[0].value = format!(
        "{:01}",
        if countdown.count == 0 { "Go!".to_string() } else { countdown.count.to_string() }
      );
    }
  }
}

pub fn ui_update_event_count (
  quick_event_data: Res<QuickEventData>,
  mut query_player: Query<&mut Text, (With<UIQuickEventPlayerCount>, Without<UIQuickEventEnemyCount>)>,
  mut query_enemy: Query<&mut Text, (With<UIQuickEventEnemyCount>, Without<UIQuickEventPlayerCount>)>
) {
  for mut text in query_player.iter_mut() {
    text.sections[0].value = format!(
      "Pressed {:02} times",
      quick_event_data.player_count
    );
  }

  for mut text in query_enemy.iter_mut() {
    text.sections[0].value = format!(
      "{:02}",
      quick_event_data.enemy_count
    );
  }
}

pub fn ui_quick_event_results (
  quick_event_data: Res<QuickEventData>,
  time: Res<Time>,
  asset_server: Res<AssetServer>,
  quick_event: Res<QuickEvent>,
  mut commands: Commands,
  mut query: Query<&mut UIQuickEventResults>,
  query_overlay: Query<Entity, With<UIQuickEventPopup>>,
  mut event_writer: EventWriter<OnQuickEventEnd>,
) {
  if quick_event.state != QuickEventState::Results {
    return;
  }

  // First run
  if query.is_empty() {
    if let Some(overlay) = query_overlay.iter().next() {
      commands.entity(overlay)
        .with_children(|parent| {
          parent.spawn_bundle(
            NodeBundle {
              style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
              },
              color: Color::rgba(0., 0., 0., 0.90).into(),
              ..Default::default()
            }
          )
          .insert(UIQuickEventResults::default())
          .with_children(|results| {

            let text = match quick_event_data.winner {
              Some(QuickEventWinner::Player) => "Player wins!",
              Some(QuickEventWinner::Enemies) => "Enemies wins!",
              _ => "",
            };

            let desc = match quick_event_data.winner {
              Some(QuickEventWinner::Player) => "You will get some unfair advantages ;)",
              Some(QuickEventWinner::Enemies) => "You weren't quick enough ;(",
              _ => "",
            };

            let text_color = match quick_event_data.winner {
              Some(QuickEventWinner::Player) => Color::GREEN,
              Some(QuickEventWinner::Enemies) => Color::RED,
              _ => Color::NONE,
            };

            results.spawn_bundle(
              TextBundle {
                style: Style {
                  margin: Rect {
                    bottom: Val::Px(24.),
                    ..Default::default()
                  },
                  ..Default::default()
                },
                text: Text::with_section(
                  text,
                  TextStyle {
                    font: asset_server.load("Fonts/KenneyPixel.ttf"),
                    font_size: 60.0,
                    color: text_color, 
                  },
                  Default::default()
                ),
                ..Default::default()
              }
            );

            results.spawn_bundle(
              TextBundle {
                text: Text::with_section(
                  desc,
                  TextStyle {
                    font: asset_server.load("Fonts/KenneyPixel.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE, 
                  },
                  Default::default()
                ),
                ..Default::default()
              }
            );

          });
        });
    }
  } else {
    if let Some(mut results) = query.iter_mut().next() {
      results.duration.tick(time.delta());

      if results.duration.finished() {
        event_writer.send(OnQuickEventEnd);
      }
    }
  }
}

pub fn ui_quick_event_despawn (
  mut commands: Commands,
  query: Query<Entity, With<UIQuickEventPopup>>,
) {
  // TODO: instead of despawning it could hide it, test?
  // oh there is a display property with a none value, it can work
  if let Some(entity) = query.iter().next() {
    commands.entity(entity).despawn_recursive();
  }
}
