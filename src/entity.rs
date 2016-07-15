use std::rc;
use std::any;
use std::sync;

use types;
use system;
use component;

pub struct Entity {
    pub id: types::EntityId,
    system: rc::Rc<system::System>,
    components: sync::RwLock<Vec<any::TypeId>>,
}

impl Entity {
    pub fn new(system: &rc::Rc<system::System>) -> rc::Rc<Entity> {
        let entity = rc::Rc::new(Entity {
            id: system.new_entity_id(),
            system: system.clone(),
            components: sync::RwLock::new(vec![]),
        });
        system.add_entity(&entity);
        entity
    }

    pub fn add<T: component::Component>(&self, comp: T) {
        // Add the component to the system and the local table
        self.system.set_component(self.id, comp);
        self.components.write().unwrap().push(any::TypeId::of::<T>());
    }
}
