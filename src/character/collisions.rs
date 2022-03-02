use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers};

use crate::{
  physics::Layer, 
  quick_event::OnQuickEvent, 
  score::{ScoreTypes, OnScorePoints}
};

use super::Health;

fn is_player(layers: CollisionLayers) -> bool {
  layers.contains_group(Layer::Player) && !layers.contains_group(Layer::Enemy)
}

fn is_player_hitbox(layers: CollisionLayers) -> bool {
  layers.contains_group(Layer::PlayerHitbox) && !layers.contains_group(Layer::Enemy)
}

fn is_enemy(layers: CollisionLayers) -> bool {
  !layers.contains_group(Layer::PlayerHitbox) && layers.contains_group(Layer::Enemy)
}

pub fn kill_enemy(
  mut commands: Commands,
  mut events: EventReader<CollisionEvent>,
  mut score_writer: EventWriter<OnScorePoints>,
) {
  events
    .iter()
    // We care about when the entities "start" to collide
    .filter(|e| e.is_started())
    .filter_map(|event| {
      let (entity_1, entity_2) = event.rigid_body_entities();
      let (layers_1, layers_2) = event.collision_layers();
      // println!("{:?}", event);
      if is_player_hitbox(layers_1) && is_enemy(layers_2) {
        Some(entity_2)
      } else if is_player_hitbox(layers_2) && is_enemy(layers_1) {
        Some(entity_1)
      } else {
        // This event is not the collision between an enemy and the player. We can ignore it.
        None
      }
    })
    .for_each(|enemy_entity| {
      commands.entity(enemy_entity).despawn();
      score_writer.send(OnScorePoints(ScoreTypes::Enemy));
    });
}

pub fn damage (
  mut events: EventReader<CollisionEvent>,
  mut health_query: Query<&mut Health>,
  mut event_writer: EventWriter<OnQuickEvent>,
) {
  events
    .iter()
    // We care about when the entities "start" to collide
    .filter(|e| e.is_started())
    .filter_map(|event| {
      let (entity_1, entity_2) = event.rigid_body_entities();
      let (layers_1, layers_2) = event.collision_layers();

      if is_player(layers_1) && is_enemy(layers_2) {
        Some(entity_1)
      } else if is_player(layers_2) && is_enemy(layers_1) {
        Some(entity_2)
      } else {
        None
      }
    })
    .for_each(|player_entity| {
      if let Ok(mut health) =  health_query.get_mut(player_entity) {
        if !health.is_invincible {
          health.decrease_health(1);
        }
        if health.health <= 0 {
          event_writer.send(OnQuickEvent);
        }
      }
    });
}
