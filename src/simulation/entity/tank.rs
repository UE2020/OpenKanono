use crate::types;
use cgmath::Vector2;
use std::any::Any;

#[derive(Debug, Default)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub lmb: bool,
    pub angle: f32,
    pub mx: i16,
    pub my: i16,
    pub rmb: bool,
}

#[derive(Debug, Clone)]
pub enum TankType {
    Player(types::Connection),
    Bot,
}

#[derive(Debug)]
pub struct Tank {
    id: types::Identifier,
    name: String,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    angle: f32,
    radius: f32,
    mass: f32,
    tank_type: TankType,
    score: u32,
    class: u16,
    color: types::Color,
    alpha: f32,
    health: f32,
    input: Input,
    speed: f32,
}

impl Tank {
    pub fn new_player(
        id: types::Identifier,
        name: String,
        x: f32,
        y: f32,
        velocity: Vector2<f32>,
        radius: f32,
        mass: f32,
        socket: tokio::sync::mpsc::UnboundedSender<warp::ws::Message>,
    ) -> Tank {
        Tank {
            id,
            name,
            position: Vector2::new(x, y),
            velocity,
            angle: 0.0,
            radius,
            mass,
            tank_type: TankType::Player(socket),
            score: 0,
            class: 0,
            color: types::Color::ChargingCyan,
            alpha: 1.0,
            health: 1000.0,
            input: Default::default(),
            speed: 1.0,
        }
    }

    pub fn input(
        &mut self,
        left: bool,
        right: bool,
        up: bool,
        down: bool,
        angle: f32,
        lmb: bool,
        mx: i16,
        my: i16,
        rmb: bool,
    ) {
        self.input.left = left;
        self.input.right = right;
        self.input.up = up;
        self.input.down = down;
        self.input.angle = angle;
        self.input.lmb = lmb;
        self.input.mx = mx;
        self.input.my = my;
        self.input.rmb = rmb;
    }
}

impl super::Entity for Tank {
    fn get_id(&self) -> types::Identifier {
        self.id
    }

    fn get_x(&self) -> f32 {
        self.position.x
    }

    fn get_y(&self) -> f32 {
        self.position.y
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn get_velocity(&self) -> Vector2<f32> {
        self.velocity
    }

    fn get_radius(&self) -> f32 {
        self.radius
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_angle(&self) -> f32 {
        self.angle
    }

    fn get_level(&self) -> u32 {
        self.score / 600
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn get_class(&self) -> u16 {
        self.class
    }

    fn get_color(&self) -> types::Color {
        self.color
    }

    fn get_alpha(&self) -> f32 {
        self.alpha
    }

    fn get_health(&self) -> f32 {
        self.health
    }

    fn send_network_packet(
        &self,
        packet: &crate::protocol::ClientboundPacket,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<warp::ws::Message>> {
        match &self.tank_type {
            TankType::Player(tx) => Ok(tx.send(warp::ws::Message::binary(packet.to_bytes()))?),
            TankType::Bot => Ok(()),
        }
    }

    fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }

    fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity = velocity;
    }

    fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    fn set_level(&mut self, level: u32) {
        self.score = level * 600;
    }

    fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    fn set_class(&mut self, class: u16) {
        self.class = class;
    }

    fn set_color(&mut self, color: types::Color) {
        self.color = color;
    }

    fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    fn set_health(&mut self, health: f32) {
        self.health = health;
    }

    fn update(&mut self, dt: f32) -> Option<fazo::Entity> {
        let current_entity = self.create_fazo_entity();
        self.velocity.x += (self.input.right as i8 - self.input.left as i8) as f32 * self.speed;
        self.velocity.y += (self.input.down as i8 - self.input.up as i8) as f32 * self.speed;
        self.position += self.velocity * dt;

        self.velocity *= 0.9;

        let new_entity = self.create_fazo_entity();
        if current_entity.x != new_entity.x
            || current_entity.y != new_entity.y
            || current_entity.radius != new_entity.radius
        {
            Some(new_entity)
        } else {
            None
        }
    }

    fn show_name(&self) -> bool {
        true
    }

    fn barrel_flash(&self) -> bool {
        false
    }

    fn shield_flash(&self) -> bool {
        false
    }

    fn can_move_through_border(&self) -> bool {
        false
    }

    fn show_health(&self) -> bool {
        true
    }

    fn networkable(&self) -> bool {
        match &self.tank_type {
            TankType::Player(_) => true,
            _ => false,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn create_fazo_entity(&self) -> fazo::Entity {
        fazo::Entity {
            id: self.id as u64,
            x: self.position.x - self.radius,
            y: self.position.y - self.radius,
            width: self.radius * 2.0,
            height: self.radius * 2.0,
            radius: self.radius,
        }
    }
}
