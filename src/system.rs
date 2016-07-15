use types;
use entity;
use component;
use std::collections;
use std::any;
use std::rc;
use std::sync;

pub struct System {
    max_entity_id: sync::RwLock<types::EntityId>,
    entities: sync::RwLock<collections::HashMap<types::EntityId, rc::Rc<entity::Entity>>>,
    components: sync::RwLock<collections::HashMap<any::TypeId, collections::HashMap<types::EntityId, Box<any::Any>>>>,
}

impl System {
    /// Create a new system
    pub fn new() -> rc::Rc<System> {
        rc::Rc::new(System {
            max_entity_id: sync::RwLock::new(0 as types::EntityId),
            entities: sync::RwLock::new(collections::HashMap::new()),
            components: sync::RwLock::new(collections::HashMap::new()),
        })
    }

    /// Add an entity to the system
    ///
    /// Should not be called by the user
    pub fn add_entity(&self, ent: &rc::Rc<entity::Entity>) {
        self.entities.write().unwrap().insert(ent.id, ent.clone());
    }

    /// Set a component for an entity
    ///
    ///
    /// Should not be called by the user
    pub fn set_component<T: component::Component>(&self, entity_id: types::EntityId, comp: T) -> Option<T> {
        let mut components = self.components.write().unwrap();
        let type_id = any::TypeId::of::<T>();
        if !components.contains_key(&type_id) {
            components.insert(type_id, collections::HashMap::new());
        }
        components.get_mut(&type_id).unwrap().insert(entity_id, Box::new(comp));
        None
    }

    pub fn borrow_component<T: component::Component>(&self, entity_id: types::EntityId) -> &T {
        self.components.read().unwrap().get(&any::TypeId::of::<T>()).unwrap().get(&entity_id).unwrap().downcast_ref::<T>().unwrap()
    }

    /// Run a closure for every entity having the specified component
    pub fn run<T>(&self, f: &Fn(&rc::Rc<entity::Entity>))
        where T: component::Component {
        for entity_id in self.components.read().unwrap().get(&any::TypeId::of::<T>()).unwrap().keys() {
            f(self.entities.read().unwrap().get(&entity_id).unwrap());
        }
    }

    pub fn new_entity_id(&self) -> types::EntityId {
        let mut id = self.max_entity_id.write().unwrap();
        *id += 1;
        *id
    }
}
