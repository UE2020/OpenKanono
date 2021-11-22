use cgmath::Vector2;

pub mod tank;

pub trait Entity: std::fmt::Debug {
    fn get_id(&self) -> crate::types::Identifer;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_position(&self) -> Vector2<f32>;
    fn get_velocity(&self) -> Vector2<f32>;
    fn get_radius(&self) -> f32;
    fn get_mass(&self) -> f32;

    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_position(&mut self, position: Vector2<f32>);
    fn set_velocity(&mut self, velocity: Vector2<f32>);
    fn set_radius(&mut self, radius: f32);
    fn set_mass(&mut self, mass: f32);

    fn update(&mut self, dt: f32);
}