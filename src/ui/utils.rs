use bevy::prelude::*;

pub fn basic_text (
  text: &str,
  size: f32,
  font: Handle<Font>,
  margin_bottom: Option<f32>
) -> TextBundle {
  let _margin_bottom = if margin_bottom.is_some() { 
    margin_bottom.unwrap() 
  } else {
    0. 
  };

  return TextBundle {
    style: Style {
      margin: Rect {
        bottom: Val::Px(_margin_bottom),
        ..Default::default()
      },
      ..Default::default()
    },
    text: Text::with_section(
      text,
      TextStyle {
        font: font,
        font_size: size,
        color: Color::WHITE,
      },
      Default::default()
    ),
    ..Default::default()
  };
}
