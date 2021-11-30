use cgmath::Vector2;
use cgmath::MetricSpace;

pub fn test_circular_collision(pos1: &Vector2<f32>, radius1: f32, pos2: &Vector2<f32>, radius2: f32) -> bool {
    let dist = pos1.distance(*pos2);
    dist < radius1 + radius2
}