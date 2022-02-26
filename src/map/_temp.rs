use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(map_startup);
  }
}

fn map_startup (
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut map_query: MapQuery,
) {
  let texture_handle: Handle<Image> = asset_server.load("Art/Map/Tilemap.png");

  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

   // Creates a new layer builder with a layer entity.
  let (mut layer_builder, _) = LayerBuilder::new(
    &mut commands,
    LayerSettings::new(
      MapSize(2, 2),
      ChunkSize(8, 8),
      TileSize(16.0, 16.0),
      TextureSize(968.0, 526.0),
    ),
    0u16,
    0u16,
  );

  layer_builder.set_all(TileBundle::default());

  let layer_entity = map_query.build_layer(
    &mut commands, 
    layer_builder, 
    texture_handle
  );

  map.add_layer(&mut commands, 0u16, layer_entity);

  commands
    .entity(map_entity)
    .insert(map)
    .insert(Transform::from_xyz(-320.0, -320.0, 0.0))
    .insert(GlobalTransform::default());
}
