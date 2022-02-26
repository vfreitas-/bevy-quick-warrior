use bevy::math::*;

const CMP_EPSILON: f32 = 0.000001;

#[allow(dead_code)]
pub fn direction_to(current: Vec2, to: Vec2) -> Vec2 {
  return Vec2::new(to.x - current.x, to.y - current.y).normalize();
}

#[allow(dead_code)]
pub fn move_toward(current: Vec2, to: Vec2, delta: f32) -> Vec2 {
  let vd = to - current;
  let len = vd.length();
  if len <= delta || len < CMP_EPSILON {
    return to;
  }

  return current + vd / len * delta;
}

#[allow(dead_code)]
pub fn reflect(velocity: Vec2, normal: Vec2) -> Vec2 {
  let mut n = Vec2::new(normal.x, normal.y);
  if !normal.is_normalized() {
    n = normal.normalize();
  }

  let dot = velocity.dot(n);
  return 2. * n * dot - velocity;
}

#[allow(dead_code)]
pub fn bounce(velocity: Vec2, normal: Vec2) -> Vec2 {
  return -(reflect(velocity, normal));
}
