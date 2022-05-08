use crate::protocol;
use crate::types;
use cgmath::Vector2;
use std::collections::HashMap;
pub mod entity;
pub mod util;
use log::*;

#[derive(Debug)]
pub struct Arena {
    width: u32,
    height: u32,
    id: types::Identifier,
    entities: HashMap<types::Identifier, Box<dyn entity::Entity>>,
    last_update: std::time::Instant,
    frame: usize,
    registered_connections: HashMap<types::Identifier, types::Connection>,
    solver: fazo::BroadSolver,
}

impl Arena {
    pub fn new(width: u32, height: u32) -> Arena {
        Arena {
            width,
            height,
            id: 3,
            entities: HashMap::new(),
            last_update: std::time::Instant::now(),
            frame: 0,
            registered_connections: HashMap::new(),
            solver: fazo::BroadSolver::new(width, height, 7),
        }
    }

    pub fn update(&mut self) {
        self.frame += 1;
        let elapsed = self.last_update.elapsed();
        self.last_update = std::time::Instant::now();
        let dt = elapsed.as_millis() as f32 / 33.0;
        if elapsed.as_millis() > 0 && self.frame % 100 == 0 {
            info!(
                "Arena cycle(elapsed={:?}, fps={}, tick={})",
                elapsed,
                1000 / elapsed.as_millis(),
                self.frame
            );
        }

        let mut disconnected_ids = vec![];

        for (_, entity) in self.entities.iter_mut() {
            let fazo_entity = match entity.update(dt) {
                Some(fazo_entity) => {
                    self.solver.mutate(&fazo_entity);
                    fazo_entity
                }
                None => entity.create_fazo_entity(),
            };

            let candidates = self.solver.solve(&fazo::Query {
                x: fazo_entity.x,
                y: fazo_entity.y,
                width: fazo_entity.width,
                height: fazo_entity.height,
            });

            for candidate in candidates {
                if candidate.id == fazo_entity.id {
                    continue;
                }
                let collision = util::test_circular_collision(
                    &cgmath::Vector2::new(
                        candidate.x + candidate.radius,
                        candidate.y + candidate.radius,
                    ),
                    candidate.radius,
                    &entity.get_position(),
                    entity.get_radius(),
                );
                if collision {
                    let angle = ((candidate.y + candidate.radius) as f32 - entity.get_y())
                        .atan2((candidate.x + candidate.radius) as f32 - entity.get_x());
                    let push_vec = Vector2::new(angle.cos(), angle.sin());
                    entity.set_velocity(entity.get_velocity() + -push_vec * 0.5);
                }
            }
        }

        for (id, socket) in self.registered_connections.iter() {
            let mut entities = vec![];
            for (_, entity) in self.entities.iter() {
                entities.push(&**entity);
            }

            let census = protocol::ClientboundPacket::Census { entities };

            if let Some(entity) = self.entities.get(id) {
                if socket
                    .send(warp::ws::Message::binary(
                        protocol::ClientboundPacket::CameraUpdate {
                            x: entity.get_x() as i32,
                            y: entity.get_y() as i32,
                            fov: 1.5,
                        }
                        .to_bytes(),
                    ))
                    .is_err()
                {
                    disconnected_ids.push(entity.get_id());
                    error!("Failed to send census packet");
                    continue;
                }
            };

            if socket
                .send(warp::ws::Message::binary(census.to_bytes()))
                .is_err()
            {
                disconnected_ids.push(*id);
                error!("Failed to send census packet");
                continue;
            }

            if socket
                .send(warp::ws::Message::binary(
                    protocol::ClientboundPacket::LeaderBoard {
                        leaderboard: vec![
                            protocol::LeaderboardEntry {
                                id: 1,
                                class: 0,
                                color: types::Color::CohortBlue,
                                name: format!(
                                    "UNIX_TIME: {}",
                                    std::time::SystemTime::now()
                                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs()
                                ),
                                score: 0,
                            },
                            protocol::LeaderboardEntry {
                                id: 2,
                                class: 0,
                                color: types::Color::CohortBlue,
                                name: format!("TICK: {}", self.frame),
                                score: 0,
                            },
                        ],
                    }
                    .to_bytes(),
                ))
                .is_err()
            {
                disconnected_ids.push(*id);
                error!("Failed to send leaderboard packet");
            }
        }

        for id in disconnected_ids {
            self.kick_connection(id);
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn entity::Entity>) {
        let r = entity.get_radius() as f32;
        self.solver.insert(&fazo::Entity {
            id: entity.get_id() as u64,
            x: entity.get_x() as f32 - r,
            y: entity.get_y() as f32 - r,
            width: r * 2.0,
            height: r * 2.0,
            radius: r,
        });
        self.entities.insert(entity.get_id(), entity);
    }

    pub fn player_spawn(&mut self, id: types::Identifier, name: String) -> bool {
        let conn = self.registered_connections.get(&id);
        let conn = match conn {
            Some(conn) => conn,
            None => return false,
        };

        if conn
            .send(warp::ws::Message::binary(
                protocol::ClientboundPacket::Joining.to_bytes(),
            ))
            .is_err()
        {
            self.kick_connection(id);
            return false;
        }

        let tank = entity::tank::Tank::new_player(
            id,
            name,
            0.0,
            0.0,
            Vector2::new(0.0, 0.0),
            100.0,
            1.0,
            conn.clone(),
        );
        self.add_entity(Box::new(tank));

        true
    }

    pub fn input(
        &mut self,
        id: types::Identifier,
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
        let entity = self.entities.get_mut(&id);
        match entity {
            Some(entity) => {
                let entity = entity.as_any_mut();
                if let Some(tank) = entity.downcast_mut::<entity::tank::Tank>() {
                    tank.input(left, right, up, down, angle, lmb, mx, my, rmb);
                }
            }
            None => {}
        }
    }

    pub fn new_connection(&mut self, conn: types::Connection) -> types::Identifier {
        let new_id = self.alloc_id();
        self.registered_connections.insert(new_id, conn);
        new_id
    }

    /// **WARNING**: This function will NOT take care of the entitiy's registered connection.
    pub fn delete_entity(&mut self, id: types::Identifier) {
        self.entities.remove(&id);
        self.solver.delete(id as u64);
    }

    pub fn kick_connection(&mut self, id: types::Identifier) -> bool {
        {
            match match self.registered_connections.get(&id) {
                Some(conn) => conn,
                None => return false,
            }
            .send(warp::ws::Message::close())
            {
                Ok(_) => {}
                Err(_) => return false,
            }
        }
        self.registered_connections.remove(&id); // remove connection
        self.delete_entity(id); // remove entity
        true
    }

    pub fn alloc_id(&mut self) -> types::Identifier {
        self.id += 1;
        self.id
    }
}
