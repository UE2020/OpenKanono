use crate::types;
use cgmath::Vector2;

pub mod tank;

pub trait Entity: Send + Sync + std::fmt::Debug {
    fn get_id(&self) -> types::Identifer;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_position(&self) -> Vector2<f32>;
    fn get_velocity(&self) -> Vector2<f32>;
    fn get_radius(&self) -> f32;
    fn get_mass(&self) -> f32;
    fn get_name(&self) -> &str;
    fn get_angle(&self) -> f32;
    fn get_level(&self) -> u32;
    fn get_score(&self) -> u32;
    fn get_class(&self) -> u16;
    fn get_color(&self) -> types::Color;
    fn get_alpha(&self) -> f32;
    fn get_health(&self) -> f32;
    fn send_network_packet(
        &self,
        packet: &crate::protocol::ClientboundPacket,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<warp::ws::Message>>;

    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_position(&mut self, position: Vector2<f32>);
    fn set_velocity(&mut self, velocity: Vector2<f32>);
    fn set_radius(&mut self, radius: f32);
    fn set_mass(&mut self, mass: f32);
    fn set_name(&mut self, name: String);
    fn set_angle(&mut self, angle: f32);
    fn set_level(&mut self, level: u32);
    fn set_score(&mut self, score: u32);
    fn set_class(&mut self, class: u16);
    fn set_color(&mut self, color: types::Color);
    fn set_alpha(&mut self, alpha: f32);
    fn set_health(&mut self, health: f32);

    fn update(&mut self, dt: f32);

    fn show_name(&self) -> bool;
    fn show_health(&self) -> bool;
    fn barrel_flash(&self) -> bool;
    fn shield_flash(&self) -> bool;
    fn can_move_through_border(&self) -> bool;
    fn networkable(&self) -> bool {
        false
    }
}
