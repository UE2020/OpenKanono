use cgmath::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TankType {
    Player,
    Bot
}

#[derive(Debug)]
pub struct Tank {
    id: crate::types::Identifer,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    radius: f32,
    mass: f32,
    tank_type: TankType,
}

impl Tank {
    pub fn new_player(id: crate::types::Identifer, x: f32, y: f32, velocity: Vector2<f32>, radius: f32, mass: f32) -> Tank {
        Tank {
            id,
            position: Vector2::new(x, y),
            velocity,
            radius,
            mass,
            tank_type: TankType::Player,
        }
    }
}

impl super::Entity for Tank {
    fn get_id(&self) -> crate::types::Identifer {
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

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}