use bevy::{
  prelude::*,
  render::camera::{DepthCalculation, ScalingMode, WindowOrigin},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(camera_setup);
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
