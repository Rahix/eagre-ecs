//! Entity-Component System
use std::any;
use std::collections;

use component;
use entity;
use error;

/// An Entity-Component System
///
/// # Example
///
/// ```
/// # use eagre_ecs::prelude::*;
///
/// // Those two traits are required for components
/// #[derive(Debug, Clone)]
/// struct MyComponent(i32);
///
/// let mut system = System::new();
/// let entity = system.new_entity();
/// system.add(entity, MyComponent(16)).unwrap();
/// system.run::<MyComponent, _>(|sys: &System, ent: Entity| {
///     println!("Entity {} has {:?}", ent, sys.borrow::<MyComponent>(ent).unwrap());
/// }).unwrap();
/// ```
#[derive(Debug)]
pub struct System {
    max_entity: entity::Entity,
    entities: collections::HashMap<entity::Entity, collections::HashSet<any::TypeId>>,
    components: collections::HashMap<any::TypeId, collections::HashMap<entity::Entity, Box<any::Any>>>,
}

impl System {
    /// Create a new Entity-Component System
    pub fn new() -> System {
        System {
            max_entity: 0,
            entities: collections::HashMap::new(),
            components: collections::HashMap::new(),
        }
    }

    /// Create a new entity
    pub fn new_entity(&mut self) -> entity::Entity {
        self.max_entity += 1;
        let new_id = self.max_entity;
        self.entities.insert(new_id, collections::HashSet::new());
        self.add::<component::All>(new_id, component::All).unwrap();
        new_id
    }

    /// Destroy an entity and all it's components
    ///
    ///
    /// Returns an Error if the Entity does not exist
    pub fn remove_entity(&mut self, ent: entity::Entity) -> error::EcsResult<()> {
        for comp in try!(self.entities.get(&ent).ok_or(error::EcsError::EntityNotFound(ent))).iter() {
            try!(self.components.get_mut(&comp).ok_or(error::EcsError::ComponentNotFound(*comp))).remove(&ent);
        }
        self.entities.remove(&ent);
        Ok(())
    }

    /// Return the number of entities, this system is currently containing
    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }

    /// Add a component to an entity
    pub fn add<T: component::Component>(&mut self, ent: entity::Entity, comp: T) -> error::EcsResult<()> {
        try!(self.set::<T>(ent, comp));
        Ok(())
    }

    /// Set a component's value for an entity
    ///
    ///
    /// Returns a previous value, if one exists
    pub fn set<T: component::Component>(&mut self, ent: entity::Entity, comp: T) -> error::EcsResult<Option<T>> {
        // Check if component is registered, if not, do so
        let type_id = any::TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            self.components.insert(type_id, collections::HashMap::new());
        }
        try!(self.entities.get_mut(&ent).ok_or(error::EcsError::EntityNotFound(ent))).insert(type_id);
        Ok(self.components.get_mut(&type_id).expect("Fatal HashMap error").insert(ent, Box::new(comp)).map(|c| *c.downcast::<T>().expect("Fatal downcast error")))
    }

    /// Remove a component from an entity
    pub fn remove<T: component::Component>(&mut self, ent: entity::Entity) -> error::EcsResult<()> {
        try!(self.components.get_mut(&any::TypeId::of::<T>()).ok_or(error::EcsError::ComponentNotFound(any::TypeId::of::<T>()))).remove(&ent);
        Ok(())
    }

    /// Borrow a component of an entity
    pub fn borrow<T: component::Component>(&self, ent: entity::Entity) -> error::EcsResult<&T> {
        Ok(try!(try!(self.components.get(&any::TypeId::of::<T>()).ok_or(error::EcsError::ComponentNotFound(any::TypeId::of::<T>())))
            .get(&ent).ok_or(error::EcsError::EntityNotFound(ent))).downcast_ref::<T>().expect("Fatal downcast error"))
    }

    /// Borrow a component of an entity mutably
    pub fn borrow_mut<T: component::Component>(&mut self, ent: entity::Entity) -> error::EcsResult<&mut T> {
        Ok(try!(try!(self.components.get_mut(&any::TypeId::of::<T>()).ok_or(error::EcsError::ComponentNotFound(any::TypeId::of::<T>())))
            .get_mut(&ent).ok_or(error::EcsError::EntityNotFound(ent))).downcast_mut::<T>().expect("Fatal downcast error"))
    }

    /// Get a copy of a component
    pub fn get<T: component::Component>(&self, ent: entity::Entity) -> error::EcsResult<T> {
        self.borrow::<T>(ent).map(|e| e.clone())
    }

    /// Test, if an entity has a component
    pub fn has<T: component::Component>(&self, ent: entity::Entity) -> bool {
        match self.components.get(&any::TypeId::of::<T>()) {
            Some(val) => val.contains_key(&ent),
            None => false,
        }
    }

    /// Run a procedure for every entity with a component
    pub fn run<T: component::Component, F: FnMut(&System, entity::Entity)>(&self, mut f: F) -> error::EcsResult<()> {
        for ent in try!(self.components.get(&any::TypeId::of::<T>()).ok_or(error::EcsError::ComponentNotFound(any::TypeId::of::<T>()))).keys() {
            f(self, *ent);
        }
        Ok(())
    }

    /// Run a procedure for every entity with a component, with changes to the system being allowed
    pub fn run_mut<T: component::Component, F: FnMut(&mut System, entity::Entity)>(&mut self, mut f: F) -> error::EcsResult<()> {
        let keys: Vec<entity::Entity> = try!(self.components.get(&any::TypeId::of::<T>()).ok_or(error::EcsError::ComponentNotFound(any::TypeId::of::<T>()))).keys().map(|e| *e).collect();
        for ent in keys {
            f(self, ent);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ::prelude::*;

    #[derive(Debug, Clone)]
    struct MyComponent {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Clone)]
    struct MySecondComponent {
        name: String,
    }

    #[test]
    fn test_system() {
        let mut system = System::new();

        // Entity 1
        let entity = system.new_entity();
        system.add(entity, MyComponent { x: 12, y: 42 }).unwrap();
        system.add(entity, MySecondComponent { name: "Foo".to_string() }).unwrap();

        // Entity 2
        let entity = system.new_entity();
        system.add(entity, MyComponent { x: 13, y: 16 }).unwrap();

        // Entity 3
        let entity = system.new_entity();
        system.add(entity, MySecondComponent { name: "Bar".to_string() }).unwrap();

        // Do something
        system.run::<MyComponent, _>(|sys: &System, ent: Entity| {
            let x = sys.borrow::<MyComponent>(ent).unwrap().x;
            let y = sys.borrow::<MyComponent>(ent).unwrap().y;
            assert!(x == 12 || x == 13);
            assert!(y == 42 || y == 16);
        }).unwrap();
        system.run::<MySecondComponent, _>(|sys: &System, ent: Entity| {
            let name = sys.get::<MySecondComponent>(ent).unwrap().name;
            assert!(name == "Foo".to_string() || name == "Bar".to_string());
        }).unwrap();
        system.run_mut::<MyComponent, _>(|sys: &mut System, ent: Entity| {
            sys.borrow_mut::<MyComponent>(ent).unwrap().x += 1;
            sys.borrow_mut::<MyComponent>(ent).unwrap().y += 1;
        }).unwrap();
        system.run::<MyComponent, _>(|sys: &System, ent: Entity| {
            let x = sys.borrow::<MyComponent>(ent).unwrap().x;
            let y = sys.borrow::<MyComponent>(ent).unwrap().y;
            assert!(x == 13 || x == 14);
            assert!(y == 43 || y == 17);
        }).unwrap();
    }
}
