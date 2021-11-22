use std::collections::HashMap;
mod entity;

#[derive(Debug)]
pub struct Arena {
    width: u32,
    height: u32,
    id: crate::types::Identifer,
    entities: HashMap<crate::types::Identifer, Box<dyn entity::Entity>>,
}

impl Arena {
    pub fn new(width: u32, height: u32) -> Arena {
        Arena {
            width,
            height,
            id: 0,
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn entity::Entity>) {
        self.entities.insert(entity.get_id(), entity);
    }

    pub fn alloc_id(&mut self) -> crate::types::Identifer {
        self.id += 1;
        self.id
    }
}