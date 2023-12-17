use std::collections::HashMap;

use ggez::mint::Point2;

use crate::GameConfig;

pub type Position = Point2<f32>;
pub type Entity = usize;
#[derive(PartialEq)]
pub enum EntityType {
    GridCell,
    Enemy,
}

pub struct ComponentStorage<T> {
    pub components: HashMap<Entity, T>,
}

impl<T> ComponentStorage<T> {
    fn new() -> ComponentStorage<T> {
        ComponentStorage {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }
}

pub struct World {
    pub next_entity_id: Entity,
    pub positions: ComponentStorage<Position>,
    pub entity_types: ComponentStorage<EntityType>
}

impl World {
    pub fn new() -> World {
        World {
            next_entity_id: 1,
            positions: ComponentStorage::new(),
            entity_types: ComponentStorage::new()
        }
    }

    pub fn create_entity(&mut self, entity_type:EntityType) -> Entity {
        let entity = self.next_entity_id;
        self.entity_types.insert(entity, entity_type);
        self.next_entity_id += 1;
        //TODO: Do we need to return the created usize or should we just insert and call it good?
        entity
    }
}

// pub struct Grid {
//     pub width: usize,
//     pub height: usize,
//     pub cell_size: f32,
// }
