use bevy::{
  prelude::*,
  render::camera::{DepthCalculation, ScalingMode, WindowOrigin},
};

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(GameState::Starting)
          .with_system(camera_setup)
      )
      .add_system_set(
        SystemSet::on_enter(GameState::GameOver)
          .with_system(camera_despawn)
      );
  }
}

#[derive(Component)]
pub struct MainCamera;

fn camera_setup(mut commands: Commands, mut windows: ResMut<Windows>) {
  let mut camera = OrthographicCameraBundle::new_2d();
  camera.orthographic_projection = OrthographicProjection {
    left: -320.,
    right: 320.,
    top: 240.,
    bottom: -240.,
    depth_calculation: DepthCalculation::ZDifference,
    scaling_mode: ScalingMode::None,
    window_origin: WindowOrigin::Center,
    scale: 1.,
    ..Default::default()
  };
  commands
    .spawn_bundle(camera)
    .insert(MainCamera);

  #[cfg(all(feature = "debug"))]
  let window = windows.get_primary_mut().unwrap();
  #[cfg(all(feature = "debug"))]
  window.set_position(IVec2::new(1550, 200));
}

pub fn camera_despawn (
  mut commands: Commands,
  query: Query<Entity, With<MainCamera>>,
) {
  if let Some(entity) = query.iter().next() {
    commands.entity(entity).despawn_recursive();
  }
}
