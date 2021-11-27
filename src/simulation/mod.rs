use crate::protocol;
use crate::types;
use cgmath::Vector2;
use std::collections::HashMap;
pub mod entity;
use log::*;

#[derive(Debug)]
pub struct Arena {
    width: u32,
    height: u32,
    id: types::Identifer,
    entities: HashMap<types::Identifer, Box<dyn entity::Entity>>,
    last_update: std::time::Instant,
    frame: usize,
    registered_connections: HashMap<types::Identifer, types::Connection>,
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
        }
    }

    pub fn update(&mut self) {
        self.frame += 1;
        let elapsed = self.last_update.elapsed();
        self.last_update = std::time::Instant::now();
        if elapsed.as_millis() > 0 && self.frame % 100 == 0 {
            info!(
                "Arena cycle(elapsed={:?}, fps={}, tick={})",
                elapsed,
                1000 / elapsed.as_millis(),
                self.frame
            );
        }

        let mut entities = vec![];
        let mut networkable_entities = vec![];

        for (id, entity) in self.entities.iter_mut() {
            entity.set_radius((self.frame as f32 / 50.0).sin().abs() * 100.0 + 100.0);
            entities.push(&**entity);
            if entity.networkable() {
                networkable_entities.push(&**entity);
            }
        }

        let census = protocol::ClientboundPacket::Census { entities };
        let mut broken_ids = vec![];

        for (id, ws) in self.registered_connections.iter() {
            if ws
                .send(warp::ws::Message::binary(census.to_bytes()))
                .is_err()
            {
                broken_ids.push(*id);
                error!("Failed to send census packet");
                continue;
            }

            if ws
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
                                name: format!(
                                    "TICK: {}", self.frame
                                ),
                                score: 0,
                            },
                        ],
                    }
                    .to_bytes(),
                ))
                .is_err()
            {
                broken_ids.push(*id);
                error!("Failed to send leaderboard packet");
            }
        }

        for id in broken_ids {
            self.kick_connection(id);
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn entity::Entity>) {
        self.entities.insert(entity.get_id(), entity);
    }

    pub fn player_spawn(&mut self, id: types::Identifer, name: String) -> bool {
        let conn = self.registered_connections.get(&id);
        let conn = match conn {
            Some(conn) => conn,
            None => return false,
        };
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

    pub fn new_connection(&mut self, conn: types::Connection) -> types::Identifer {
        let new_id = self.alloc_id();
        self.registered_connections.insert(new_id, conn);
        new_id
    }

    pub fn kick_connection(&mut self, id: types::Identifer) -> bool {
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
        self.entities.remove(&id); // remove entity
        true
    }

    pub fn alloc_id(&mut self) -> types::Identifer {
        self.id += 1;
        self.id
    }
}
