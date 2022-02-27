use bevy::prelude::*;

use crate::quick_event::QuickEventData;

use super::UIRootNode;

#[derive(Component)]
pub struct UIQuickEventPopup;

#[derive(Component)]
pub struct UIQuickEventPlayerCount;

pub fn ui_quick_event_spawn (
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
  mut query: Query<Entity, With<UIRootNode>>,
) {
  // let sprite = SpriteSheetBundle {
  //   texture_atlas: 
  //   ..Default::default()
  // };


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
                    "Press the random key assigned to the player fastest as you can to win the duel!",
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
                  ImageBundle {
                    style: Style {
                      margin: Rect {
                        bottom: Val::Px(24.),
                        ..Default::default()
                      },
                      ..Default::default()
                    },
                    image: UiImage(asset_server.load("Art/UI/keybinds.png")),
                    ..Default::default()
                  }
                );

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
                  ImageBundle {
                    image: UiImage(asset_server.load("Art/UI/keybinds.png")),
                    ..Default::default()
                  }
                );
              });

              
            });
          });
        });
      }); // 
  }
}

pub fn ui_update_event_count (
  quick_event_data: Res<QuickEventData>,
  mut query: Query<&mut Text, With<UIQuickEventPlayerCount>>
) {
  for mut text in query.iter_mut() {
    text.sections[0].value = format!("Press {:?} - {:02} times", quick_event_data.key, quick_event_data.count);
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
